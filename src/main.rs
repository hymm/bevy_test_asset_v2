use bevy::{prelude::*, asset::AssetLoader};
use bevy::asset::saver::AssetSaver;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin::processed_dev()))
        .run();
}

struct TextureAtlasSaver {
    atlases: HashMap<&'static str, Vec<Handle<Image>>>,
}

#[derive(Serialize, Deserialize, Default)]
struct TextureAtlasSaverSettings {
    target_atlas: String,
}

impl AssetSaver for TextureAtlasSaver {
    type Asset = Image;

    type Settings = TextureAtlasSaverSettings;

    type OutputLoader = TextureAtlasLoader;

    fn save<'a>(
        &'a self,
        writer: &'a mut bevy::asset::io::Writer,
        asset: &'a Self::Asset,
        settings: &'a Self::Settings,
    ) -> bevy::utils::BoxedFuture<'a, Result<<Self::OutputLoader as bevy::asset::AssetLoader>::Settings, anyhow::Error>> {
        todo!()
    }
}

struct TextureAtlasLoader {

}

#[derive(Serialize, Deserialize, Default)]
struct TextureAtlasLoaderSettings {}

impl AssetLoader for TextureAtlasLoader {
    type Asset = Image;

    type Settings = TextureAtlasLoaderSettings;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        todo!()
    }

    fn extensions(&self) -> &[&str] {
        todo!()
    }
}