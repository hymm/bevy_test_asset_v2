use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use serde::{Deserialize, Serialize};

pub struct TextureAtlasLoader;

#[derive(Serialize, Deserialize, Default)]
pub struct TextureAtlasLoaderSettings {
    areas: Vec<Rect>,
    image: String,
}

impl AssetLoader for TextureAtlasLoader {
    type Asset = TextureAtlas;

    type Settings = TextureAtlasLoaderSettings;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ron: TextureAtlasLoaderSettings = ron::de::from_bytes(&bytes)?;

            let image = load_context.load_direct(&ron.image).await?;
            let image: Image = image.take().unwrap();
            let dimensions = image.size();

            let handle = load_context.add_labeled_asset(ron.image, image);

            let mut texture_atlas = TextureAtlas::new_empty(handle, dimensions);
            for area in ron.areas {
                texture_atlas.add_texture(area);
            }

            Ok(texture_atlas)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["texture_atlas"]
    }
}
