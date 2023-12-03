use bevy::prelude::*;

use crate::{
    constants::*,
    maze::Maze,
    sprites::LevelSpriteSheet,
};

//
// Declara plugin do nível
//

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_level);
    }
}

//
// Spawna as sprites do nível (paredes, chão, etc)
//

pub fn spawn_level(
    mut commands: Commands,
    maze: Res<Maze>,
    level_sprite_sheet: Res<LevelSpriteSheet>,
) {
    // obtém dimensões do labirinto
    let rows = maze.matrix.rows() as i32;
    let cols = maze.matrix.cols() as i32;

    // itera sobre as dimensões do labirinto
    for i in 0..rows {
        for j in 0..cols {
            // calcula posição em pixels
            let x = j as f32 * LEVEL_SPRITE_SIZE.0 * LEVEL_SPRITE_SCALE;
            let y_range = (cols as f32 * (LEVEL_MARGIN + 1.0)) - (-cols as f32 * LEVEL_MARGIN);
            let y =  y_range - (i as f32 * LEVEL_SPRITE_SIZE.1 * LEVEL_SPRITE_SCALE);

            // gera transformação da sprite
            let translation = Vec3::new(x, y, -10.0);
            let transform = Transform::from_translation(translation)
                .with_scale(Vec3::new(LEVEL_SPRITE_SCALE, LEVEL_SPRITE_SCALE, LEVEL_SPRITE_SCALE));

            // associa elemento do labirinto com sprite
            match *maze.matrix.get(i as usize, j as usize).unwrap() {
                // se 0, spawna textura no index 4 (parede)
                0 => {
                    commands.spawn(SpriteSheetBundle {
                        texture_atlas: level_sprite_sheet.0.to_owned(),
                        sprite: TextureAtlasSprite::new(4),
                        transform,
                        ..default()
                    });
                },
                // se 1, spawna textura no index 8 (chão)
                1 => {
                    commands.spawn(SpriteSheetBundle {
                        texture_atlas: level_sprite_sheet.0.to_owned(),
                        sprite: TextureAtlasSprite::new(8),
                        transform,
                        ..default()
                    });
                },
                // se 2, spawna textura nos indices 8 (chão) e 1 (baú)
                2 => {
                    commands.spawn(SpriteSheetBundle {
                        texture_atlas: level_sprite_sheet.0.to_owned(),
                        sprite: TextureAtlasSprite::new(8),
                        transform,
                        ..default()
                    });
                    let mut transform = transform.clone();
                    transform.translation.z = -9.0;
                    commands.spawn(SpriteSheetBundle {
                        texture_atlas: level_sprite_sheet.0.to_owned(),
                        sprite: TextureAtlasSprite::new(1),
                        transform,
                        ..default()
                    });
                },
                // se qualquer outro número, spawna texturas nos indices 8 (chão) e 9 (escada)
                _ => {
                    commands.spawn(SpriteSheetBundle {
                        texture_atlas: level_sprite_sheet.0.to_owned(),
                        sprite: TextureAtlasSprite::new(8),
                        transform,
                        ..default()
                    });
                    let mut transform = transform.clone();
                    transform.translation.z = -9.0;
                    commands.spawn(SpriteSheetBundle {
                        texture_atlas: level_sprite_sheet.0.to_owned(),
                        sprite: TextureAtlasSprite::new(9),
                        transform,
                        ..default()
                    });
                },
            }
        }
    }
}