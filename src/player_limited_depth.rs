use bevy::prelude::*;
//use bevy_rapier2d::prelude::*;
use pyo3::prelude::*;
use simple_matrix::Matrix;

use crate::{
    constants::*,
    maze::Maze,
    sprites::CharactersSpriteSheet,
};

pub struct LimitedDepthPlayerPlugin;

impl Plugin for LimitedDepthPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_limited_depth_player);
    }
}

fn calculate_limited_depth(
    matrix: &Matrix<u8>,
    exits: &Vec<(usize, usize)>,
    start_position: (usize, usize),
) -> PyResult<(f64, Vec<(usize, usize)>)> {
    let limited_depth_code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/limited_depth.py"
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

        let limited_depth_module = PyModule::from_code(
            py,
            limited_depth_code,
            "python.limited_depth",
            "python.limited_depth")?;

        let result: (f64, Vec<(usize, usize)>) = limited_depth_module
            .getattr("calculate_limited_depth")?
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


#[derive(Component)]
pub struct LimitedDepthPlayer {
    timer: Timer,
}


pub fn spawn_limited_depth_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    maze: Res<Maze>,
    characters_sprite_sheet: Res<CharactersSpriteSheet>,
) {
    if let Some(start_position) = maze.limited_depth_start {
        match calculate_limited_depth(
            &maze.matrix,
            &maze.exits,
            start_position,
        ) {
            Ok((time, path)) => {
                let text_style = TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 18.0,
                    ..default()
                };

                let rows = maze.matrix.rows() as f32;
                let cols = maze.matrix.cols() as f32;

                let i = (rows - 1.0) / 2.0;
                let j = (cols - 1.0) / 2.0;

                let x = j * TILE_SIZE * SCALE;
                let y_range = (cols * (MAP_PADDING + 1.0)) - (-cols * MAP_PADDING);
                let y =  y_range - (i * TILE_SIZE * SCALE);

                let translation = Vec3::new(x, y, -10.0);
                let transform = Transform::from_translation(translation)
                    .with_scale(Vec3::new(SCALE, SCALE, SCALE));

                commands.spawn(
                    TextBundle::from_section(
                        format!(
                            "Bob Gótico dos Santos\nBusca em Profundidade Limitada\nTempo de Processamento: {} ms\nTamanho da Solução: {}",
                            time / 1_000_000.0,
                            path.len(),
                        ),
                        text_style.clone(),
                    )
                    .with_text_alignment(TextAlignment::Left)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        ..default()
                    }),
                );

                let player = LimitedDepthPlayer {
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                };

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
            }
        }
    }
}