use bevy::prelude::*;

use crate::constants::*;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, load_level_sprite_sheet)
            .add_systems(PreStartup, load_amelia_sprite_sheet)
            .add_systems(PreStartup, load_bob_sprite_sheet);
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
        Vec2::new(LEVEL_SPRITE_SIZE.0, LEVEL_SPRITE_SIZE.1), 
        10, 
        10, 
        None, 
        None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(LevelSpriteSheet(texture_atlas_handle));
}

#[derive(Resource)]
pub struct AmeliaSpriteSheet(pub Handle<TextureAtlas>);

fn load_amelia_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("sprites/amelia.png");
    let texture_atlas: TextureAtlas = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(PLAYER_SPRITE_SIZE.0, PLAYER_SPRITE_SIZE.1), 
        24, 
        2,
        None, 
        None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(AmeliaSpriteSheet(texture_atlas_handle));
}

#[derive(Resource)]
pub struct BobSpriteSheet(pub Handle<TextureAtlas>);

fn load_bob_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("sprites/bob.png");
    let texture_atlas: TextureAtlas = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(PLAYER_SPRITE_SIZE.0, PLAYER_SPRITE_SIZE.1), 
        24, 
        2,
        None, 
        None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(BobSpriteSheet(texture_atlas_handle));
}