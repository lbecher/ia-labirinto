use bevy::{
    /*core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },*/
    prelude::*,
};

use crate::{
    constants::*,
    maze::Maze,
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
    let y_range = (cols * (MAP_PADDING + 1.0)) - (-cols * MAP_PADDING);
    let y =  y_range - (i * LEVEL_SPRITE_SIZE.1 * LEVEL_SPRITE_SCALE);

    let translation = Vec3::new(x, y, -10.0);
    let transform = Transform::from_translation(translation);
    
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                //hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            transform,
            //tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        //BloomSettings::default(), // 3. Enable bloom for the camera
    ));
}

fn camera_movement(
    input: Res<Input<KeyCode>>,
    mut transform_query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = transform_query.single_mut();

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

        transform.translation.x += velocity.x * SPEED * 2.0;
        transform.translation.y += velocity.y * SPEED * 2.0;
    }
}