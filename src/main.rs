mod camera;
mod constants;
mod level;
mod maze;
mod player_a_star;
mod player_limited_depth;
mod sprites;

use bevy::{
    prelude::*,
    window::{
        WindowMode,
        WindowTheme,
    },
};

use crate::{
    camera::CameraPlugin,
    //constants::*,
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
                    mode: WindowMode::BorderlessFullscreen,
                    //resolution: (WIDTH, HEIGHT).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, controls_text)
        .add_plugins(SpritesPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MazePlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(AStarPlayerPlugin)
        .add_plugins(LimitedDepthPlayerPlugin)
        .run();
}

//
// Spawna texto explicando os controles
//

fn controls_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 18.0,
        ..default()
    };

    commands.spawn(
        TextBundle::from_section(
            "Controles:\n - Movimentar a câmera: Setas ou WASD\n - Seguir a Amelia: Manter 1 pressionado\n - Seguir o Bob: Manter 2 pressionado\n - Sair da aplicação: Alt + F4",
            text_style.clone(),
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}
