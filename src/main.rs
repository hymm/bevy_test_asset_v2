mod texture_atlas_loader;
mod texture_atlas_builder;
mod texture_atlas_builder_loader;

use bevy::prelude::*;
use texture_atlas_builder_loader::TextureAtlasBuilderPlugin;
use texture_atlas_loader::TextureAtlasLoader;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin::processed_dev()))
        .add_plugins(TextureAtlasBuilderPlugin)
        .register_asset_loader(TextureAtlasLoader)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_atlas = asset_server.load("images/suv.texture_atlas");

    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas,
        ..default()
    });

    let texture_atlas = asset_server.load("images/tiles.atlas_builder");

    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas,
        ..default()
    });
}