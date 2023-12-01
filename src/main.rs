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

use crate::{
    camera::CameraPlugin,
    constants::*,
    level::LevelPlugin,
    maze::MazePlugin,
    player_a_star::AStarPlayerPlugin,
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
                    title: "Profundidade Limitada X A Estrela".into(),
                    window_theme: Some(WindowTheme::Dark),
                    resolution: (WIDTH, HEIGHT).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(SpritesPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MazePlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(AStarPlayerPlugin)
        .add_plugins(LimitedDepthPlayerPlugin)
        .run();
}