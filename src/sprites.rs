use bevy::prelude::*;

use crate::constants::*;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, load_level_sprite_sheet)
            .add_systems(PreStartup, load_characters_sprite_sheet);
    }
}

#[derive(Resource)]
pub struct LevelSpriteSheet(pub Handle<TextureAtlas>);

fn load_level_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("sprites/dungeon.png");
    let texture_atlas: TextureAtlas = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(TILE_SIZE, TILE_SIZE), 
        10, 
        10, 
        None, 
        None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(LevelSpriteSheet(texture_atlas_handle));
}

#[derive(Resource)]
pub struct CharactersSpriteSheet(pub Handle<TextureAtlas>);

fn load_characters_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("sprites/characters.png");
    let texture_atlas: TextureAtlas = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(TILE_SIZE, TILE_SIZE), 
        12, 
        4, 
        None, 
        None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(CharactersSpriteSheet(texture_atlas_handle));
}