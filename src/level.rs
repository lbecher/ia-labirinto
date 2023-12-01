use bevy::prelude::*;

use crate::{
    constants::*,
    maze::Maze,
    sprites::LevelSpriteSheet,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_level);
    }
}

pub fn spawn_level(
    mut commands: Commands,
    maze: Res<Maze>,
    level_sprite_sheet: Res<LevelSpriteSheet>,
) {
    let rows = maze.matrix.rows() as i32;
    let cols = maze.matrix.cols() as i32;

    for i in (-rows * MAP_PADDING as i32)..(rows * (MAP_PADDING as i32 + 1)) {
        for j in (-cols * MAP_PADDING as i32)..(cols * (MAP_PADDING as i32 + 1)) {
            let x = j as f32 * LEVEL_SPRITE_SIZE.0 * LEVEL_SPRITE_SCALE;
            let y_range = (cols as f32 * (MAP_PADDING + 1.0)) - (-cols as f32 * MAP_PADDING);
            let y =  y_range - (i as f32 * LEVEL_SPRITE_SIZE.1 * LEVEL_SPRITE_SCALE);

            let translation = Vec3::new(x, y, -10.0);
            let transform = Transform::from_translation(translation)
                .with_scale(Vec3::new(LEVEL_SPRITE_SCALE, LEVEL_SPRITE_SCALE, LEVEL_SPRITE_SCALE));

            if i >= 0 && i < rows && j >= 0 && j < rows {
                match *maze.matrix.get(i as usize, j as usize).unwrap() {
                    0 => {
                        commands.spawn(SpriteSheetBundle {
                            texture_atlas: level_sprite_sheet.0.to_owned(),
                            sprite: TextureAtlasSprite::new(1),
                            transform,
                            ..default()
                        });
                    },
                    _ => {
                        commands.spawn(SpriteSheetBundle {
                            texture_atlas: level_sprite_sheet.0.to_owned(),
                            sprite: TextureAtlasSprite::new(22),
                            transform,
                            ..default()
                        });
                    },
                }
            } else {
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