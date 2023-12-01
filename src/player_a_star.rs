use bevy::prelude::*;
use pyo3::prelude::*;
use simple_matrix::Matrix;

use crate::{
    constants::*,
    maze::Maze,
    sprites::AmeliaSpriteSheet,
};

pub struct AStarPlayerPlugin;

impl Plugin for AStarPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_a_star_player)
            .add_systems(Update, movement_a_star_player);
    }
}

fn calculate_a_star(
    matrix: &Matrix<u8>,
    exits: &Vec<(usize, usize)>,
    start_position: (usize, usize),
) -> PyResult<(f64, Vec<(usize, usize)>)> {
    let a_star_code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/a_star.py"
    ));

    // Initialize Python in a thread-safe manner
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let mut vec_matrix: Vec<Vec<u8>> = Vec::new();
        for i in 0..matrix.rows() {
            vec_matrix.push(Vec::new());
            for j in 0..matrix.cols() {
                vec_matrix[i].push(*matrix.get(i, j).unwrap());
            }
        }

        let a_star_module = PyModule::from_code(
            py,
            a_star_code,
            "python.a_star",
            "python.a_star")?;

        let result: (f64, Vec<(usize, usize)>) = a_star_module
            .getattr("calculate_a_star")?
            .call((
                vec_matrix,
                matrix.rows(),
                matrix.cols(),
                exits.to_vec(),
                start_position,
            ), None)?
            .extract()?;
        Ok(result)
    })
}

pub enum AStarPlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct AStarPlayer {
    path: Vec<(usize, usize)>,
    timer: Timer,
    frame_offset: usize,
    current_frame: usize,
    first_frame: usize,
    last_frame: usize,
    direction: AStarPlayerDirection,
}

impl AStarPlayer {
    pub fn new(path: Vec<(usize, usize)>, timer: Timer) -> Self {
        Self { 
            path,
            timer,
            frame_offset: 6,
            current_frame: 0, 
            first_frame: 0, 
            last_frame: 5,
            direction: AStarPlayerDirection::Down,
        }
    }

    pub fn get_current_frame(&self) -> usize {
        self.current_frame + self.frame_offset
    }

    pub fn update_sprite(&mut self, time: &Time) {
        self.timer.tick(time.delta());
        if self.timer.just_finished() {
            self.current_frame = if self.current_frame == self.last_frame {
                self.first_frame
            } else {
                self.current_frame + 1
            };
        }
    }

    pub fn update_animation(&mut self, velocity: Vec2) {
        if velocity == Vec2::ZERO {
            self.frame_offset = 0;
        } else {
            self.frame_offset = 24;

            if velocity.x > 0.0 {
                self.direction = AStarPlayerDirection::Right;
            }
            else if velocity.x < 0.0 {
                self.direction = AStarPlayerDirection::Left;
            }
            else if velocity.y > 0.0 {
                self.direction = AStarPlayerDirection::Down;
            }
            else {
                self.direction = AStarPlayerDirection::Up;
            }
        }

        match self.direction {
            AStarPlayerDirection::Up => {
                self.frame_offset += 18;
            }
            AStarPlayerDirection::Down => {
                self.frame_offset += 6;
            }
            AStarPlayerDirection::Left => {
                self.frame_offset += 12;
            }
            AStarPlayerDirection::Right => {
                self.frame_offset += 0;
            }
        };
    }
}


pub fn spawn_a_star_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    maze: Res<Maze>,
    characters_sprite_sheet: Res<AmeliaSpriteSheet>,
) {
    if let Some(start_position) = maze.a_star_start {
        let text_style = TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 18.0,
            ..default()
        };

        match calculate_a_star(
            &maze.matrix,
            &maze.exits,
            start_position,
        ) {
            Ok((time, path)) => {
                let path = path;

                let cols = maze.matrix.cols() as f32;

                let i = start_position.0 as f32;
                let j = start_position.1 as f32;

                let x = j * PLAYER_SPRITE_SIZE.0 * PLAYER_SPRITE_SCALE * (PLAYER_SPRITE_SIZE.1 / PLAYER_SPRITE_SIZE.0);
                let y_range = (cols * (MAP_PADDING + 1.0)) - (-cols * MAP_PADDING);
                let y =  y_range - (i * PLAYER_SPRITE_SIZE.1 * PLAYER_SPRITE_SCALE);

                let translation = Vec3::new(x, y, 0.0);
                let transform = Transform::from_translation(translation)
                    .with_scale(Vec3::new(PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE));

                commands.spawn(
                    TextBundle::from_section(
                        format!(
                            "Amelia Amarela de Oliveira\nA* (A Estrela)\nTempo de Processamento: {} ms\nTamanho da Solução: {}",
                            time / 1_000_000.0,
                            path.len(),
                        ),
                        text_style.clone(),
                    )
                    .with_text_alignment(TextAlignment::Left)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..default()
                    }),
                );

                let player = AStarPlayer::new(
                    path,
                    Timer::from_seconds(0.1, TimerMode::Repeating));

                commands.spawn((
                    player,
                    SpriteSheetBundle {
                        texture_atlas: characters_sprite_sheet.0.to_owned(),
                        sprite: TextureAtlasSprite::new(0),
                        transform,
                        ..default()
                    },
                ));
            },
            Err(err) => {
                eprintln!("{}", err);
                commands.spawn(
                    TextBundle::from_section(
                        "Não foi possível inicializar o player Amelia\nA* (A Estrela)",
                        text_style.clone(),
                    )
                    .with_text_alignment(TextAlignment::Left)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..default()
                    }),
                );
            }
        }
    }
}

fn movement_a_star_player(
    time: Res<Time>,
    maze: Res<Maze>,
    mut query: Query<(
        &mut AStarPlayer,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
) {
    if let Ok((
        mut player,
        mut texture_atlas_sprite,
        mut transform,
    )) = query.get_single_mut() {
        let path_len = player.path.len();

        let mut velocity = Vec2::ZERO;

        if path_len > 0 {
            let (i, j) = player.path[path_len - 1];

            let cols = maze.matrix.cols() as f32;

            let next_x = j as f32 * PLAYER_SPRITE_SIZE.0 * PLAYER_SPRITE_SCALE * (PLAYER_SPRITE_SIZE.1 / PLAYER_SPRITE_SIZE.0);
            let y_range = (cols * (MAP_PADDING + 1.0)) - (-cols * MAP_PADDING);
            let next_y =  y_range - (i as f32 * PLAYER_SPRITE_SIZE.1 * PLAYER_SPRITE_SCALE);

            let x_difference = next_x - transform.translation.x;
            let y_difference = next_y - transform.translation.y;

            if x_difference.abs() > 0.0 {
                velocity.x = x_difference / x_difference.abs();
            }
            if y_difference.abs() > 0.0 {
                velocity.y = y_difference / y_difference.abs();
            }

            if x_difference.abs() < SPEED && y_difference.abs() < SPEED {
                player.path.pop();
                transform.translation.x = next_x;
                transform.translation.y = next_y;
            }
            else {
                velocity = velocity.normalize();
                transform.translation.x += SPEED * velocity.x;
                transform.translation.y += SPEED * velocity.y;
            }
        }

        player.update_animation(velocity);
        player.update_sprite(&time);
        texture_atlas_sprite.index = player.get_current_frame();
    }
}
