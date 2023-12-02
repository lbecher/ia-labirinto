use bevy::prelude::*;

use crate::{
    constants::*,
    maze::Maze,
    player_limited_depth::LimitedDepthPlayer,
    player_a_star::AStarPlayer,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, camera_spawn)
            .add_systems(Update, camera_movement);
    }
}

fn camera_spawn(
    mut commands: Commands,
    maze: Res<Maze>,
) {
    let rows = maze.matrix.rows() as f32;
    let cols = maze.matrix.cols() as f32;

    let i = (rows - 1.0) / 2.0;
    let j = (cols - 1.0) / 2.0;

    let x = j * LEVEL_SPRITE_SIZE.0 * LEVEL_SPRITE_SCALE;
    let y_range = (cols * (LEVEL_MARGIN + 1.0)) - (-cols * LEVEL_MARGIN);
    let y =  y_range - (i * LEVEL_SPRITE_SIZE.1 * LEVEL_SPRITE_SCALE);

    let translation = Vec3::new(x, y, -10.0);
    let transform = Transform::from_translation(translation);
    
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                ..default()
            },
            transform,
            ..default()
        },
    ));
}

fn camera_movement(
    input: Res<Input<KeyCode>>,
    a_star_query: Query<(&AStarPlayer, &Transform), Without<Camera>>,
    limited_depth_query: Query<(&LimitedDepthPlayer, &Transform), Without<Camera>>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    let (_, mut camera_transform) = camera_query.single_mut();

    let mut follow_player: Option<(f32, f32)> = None;

    if input.pressed(KeyCode::Key1) {
        if let Ok((_, a_star_transform)) = a_star_query.get_single() {
            follow_player = Some((
                a_star_transform.translation.x,
                a_star_transform.translation.y,
            ));
        }
    }
    else if input.pressed(KeyCode::Key2) {
        if let Ok((_, limilimited_depth_transform)) = limited_depth_query.get_single() {
            follow_player = Some((
                limilimited_depth_transform.translation.x,
                limilimited_depth_transform.translation.y,
            ));
        }
    }

    if let Some(translation) = follow_player {
        camera_transform.translation.x = translation.0;
        camera_transform.translation.y = translation.1;
    }
    else {
        let mut velocity = Vec2::ZERO;

        if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
            velocity.x -= 1.0;
        }
        if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
            velocity.x += 1.0;
        }

        if input.pressed(KeyCode::Down) || input.pressed(KeyCode::S) {
            velocity.y -= 1.0;
        }
        if input.pressed(KeyCode::Up) || input.pressed(KeyCode::W) {
            velocity.y += 1.0;
        }

        if velocity != Vec2::ZERO {
            velocity = velocity.normalize();

            camera_transform.translation.x += velocity.x * SPEED * 2.0;
            camera_transform.translation.y += velocity.y * SPEED * 2.0;
        }
    }
}