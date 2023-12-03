use bevy::prelude::*;

use crate::{
    constants::*,
    maze::Maze,
    player_limited_depth::LimitedDepthPlayer,
    player_a_star::AStarPlayer,
};

//
// Declara plugin da câmera
//

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, camera_spawn)
            .add_systems(Update, camera_movement);
    }
}

//
// Spawna câmera
//

fn camera_spawn(
    mut commands: Commands,
    maze: Res<Maze>,
) {
    // obtém dimensões do labirinto
    let rows = maze.matrix.rows() as f32;
    let cols = maze.matrix.cols() as f32;

    // define posição da câmera: o centro do labirinto
    let i = (rows - 1.0) / 2.0;
    let j = (cols - 1.0) / 2.0;

    // converte coordenadas do labirinto em coordenadas da tela
    let x = j * LEVEL_SPRITE_SIZE.0 * LEVEL_SPRITE_SCALE;
    let y_range = (cols * (LEVEL_MARGIN + 1.0)) - (-cols * LEVEL_MARGIN);
    let y =  y_range - (i * LEVEL_SPRITE_SIZE.1 * LEVEL_SPRITE_SCALE);

    // cria objeto de transformação com base nas coordenadas de tela
    let translation = Vec3::new(x, y, -10.0);
    let transform = Transform::from_translation(translation);
    
    // spawna câmera
    commands.spawn(Camera2dBundle {
        camera: Camera {
            ..default()
        },
        transform,
        ..default()
    });
}

//
// Aplica movimetação à câmera
//

fn camera_movement(
    input: Res<Input<KeyCode>>,
    a_star_query: Query<(&AStarPlayer, &Transform), Without<Camera>>,
    limited_depth_query: Query<(&LimitedDepthPlayer, &Transform), Without<Camera>>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    // obtém transformação da câmera
    let (_, mut camera_transform) = camera_query.single_mut();

    // variável que controla se a câmera está seguindo o player
    let mut follow_player: Option<(f32, f32)> = None;

    // se tecla 1 for pressionada
    if input.pressed(KeyCode::Key1) || input.pressed(KeyCode::Numpad1) {
        // tenta obter a transformação do player a star
        if let Ok((_, a_star_transform)) = a_star_query.get_single() {
            // se o player existe, aplica a transformação dele na variável
            follow_player = Some((
                a_star_transform.translation.x,
                a_star_transform.translation.y,
            ));
        }
    }

    // se tecla 1 for pressionada
    else if input.pressed(KeyCode::Key2) || input.pressed(KeyCode::Numpad2) {
        // tenta obter a transformação do player limited depth
        if let Ok((_, limilimited_depth_transform)) = limited_depth_query.get_single() {
            // se o player existe, aplica a transformação dele na variável
            follow_player = Some((
                limilimited_depth_transform.translation.x,
                limilimited_depth_transform.translation.y,
            ));
        }
    }

    // se a câmera está seguindo um player
    if let Some(translation) = follow_player {
        // aplica a transformação do player na câmera
        camera_transform.translation.x = translation.0;
        camera_transform.translation.y = translation.1;
    }

    // se não, verifica se o usuário está tentando mover a câmera
    else {
        let mut velocity = Vec2::ZERO;

        // incrementa velocidade das teclas de movimentação
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

        // se a velocidade for diferente de zero, aplica à câmera
        if velocity != Vec2::ZERO {
            velocity = velocity.normalize();
            camera_transform.translation.x += velocity.x * SPEED * 2.0;
            camera_transform.translation.y += velocity.y * SPEED * 2.0;
        }
    }
}