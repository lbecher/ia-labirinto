mod camera;
mod constants;
mod level;
mod maze;
mod player_a_star;
mod player_limited_depth;
mod sprites;

use bevy::{
    prelude::*,
    window::WindowTheme,
};
use bevy_rapier2d::prelude::*;

use crate::{
    camera::CameraPlugin,
    constants::*,
    level::LevelPlugin,
    maze::MazePlugin,
    //player_a_star::PlayerAStarPlugin,
    player_limited_depth::LimitedDepthPlayerPlugin,
    sprites::SpritesPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Primeiro Trabalho de IA".into(),
                    window_theme: Some(WindowTheme::Dark),
                    resolution: (WIDTH, HEIGHT).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(SpritesPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MazePlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(LimitedDepthPlayerPlugin)
        .run();
}