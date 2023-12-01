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

    for i in (-MAP_PADDING as i32)..(rows + MAP_PADDING as i32) {
        for j in (-MAP_PADDING as i32)..(cols + MAP_PADDING as i32) {
            // calcula posição em pixels
            let x = j as f32 * LEVEL_SPRITE_SIZE.0 * LEVEL_SPRITE_SCALE;
            let y_range = (cols as f32 * (MAP_PADDING + 1.0)) - (-cols as f32 * MAP_PADDING);
            let y =  y_range - (i as f32 * LEVEL_SPRITE_SIZE.1 * LEVEL_SPRITE_SCALE);

            // gera transformação da sprite
            let translation = Vec3::new(x, y, -10.0);
            let transform = Transform::from_translation(translation)
                .with_scale(Vec3::new(LEVEL_SPRITE_SCALE, LEVEL_SPRITE_SCALE, LEVEL_SPRITE_SCALE));

            // se posição estiver dentro da área do labirinto, spawna de acordo com o tipo
            if i >= 0 && i < rows && j >= 0 && j < cols {
                // associa número da matriz com o tipo de sprite
                match *maze.matrix.get(i as usize, j as usize).unwrap() {
                    // se 0, spawna textura no index 1 (parede)
                    0 => {
                        commands.spawn(SpriteSheetBundle {
                            texture_atlas: level_sprite_sheet.0.to_owned(),
                            sprite: TextureAtlasSprite::new(1),
                            transform,
                            ..default()
                        });
                    },
                    // se qualquer outro número, spawna textura no index 22 (chão)
                    _ => {
                        commands.spawn(SpriteSheetBundle {
                            texture_atlas: level_sprite_sheet.0.to_owned(),
                            sprite: TextureAtlasSprite::new(22),
                            transform,
                            ..default()
                        });
                    },
                }
            }
            
            // se não, spawna textura da borda do labirinto (index 78)
            else {
                commands.spawn(SpriteSheetBundle {
                    texture_atlas: level_sprite_sheet.0.to_owned(),
                    sprite: TextureAtlasSprite::new(78),
                    transform,
                    ..default()
                });
            }
        }
    }
}