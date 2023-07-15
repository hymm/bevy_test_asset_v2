use bevy::asset::saver::AssetSaver;
use bevy::asset::AsyncReadExt;
use bevy::reflect::TypePath;
use bevy::utils::HashMap;

#[derive(Asset, TypePath)]
struct AtlasImage {
    texture: Handle<Image>,
    named_areas: HashMap<String, usize>,
    areas: Vec<Rect>,
}

/// loads a texture atlas from a config file
struct TextureAtlasBuilderLoader {}

#[derive(Serialize, Deserialize, Default)]
struct TextureAtlasConfigLoaderSettings {}

#[derive(Serialize, Deserialize, Default)]
struct TextureAtlasBuilder {
    source_images: Vec<String>,
}

impl AssetLoader for TextureAtlasBuilderLoader {
    type Asset = TextureAtlasBuilder;

    type Settings = TextureAtlasLoaderSettings;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ron: TextureAtlasBuilder = ron::de::from_bytes(&bytes)?;

            let builder = bevy::prelude::TextureAtlasBuilder::default();


            for src in ron.source_images {
                let loaded = load_context.load_direct(&src).await?;
                let image = loaded.get::<Image>().unwrap();

            }

            todo!()
        })
    }

    fn extensions(&self) -> &[&str] {
        &["texture_atlas_builder"]
    }
}


struct TextureAtlasBuilderSaver {}

impl AssetSaver for TextureAtlasBuilderSaver {
    type Asset = TextureAtlasBuilder;
    type Settings = TextureAtlasBuilderSaverSettings;
    type OutputLoader = TextureAtlasLoader;

    fn save<'a>(
        &'a self,
        writer: &'a mut Writer,
        asset: &'a S
    )
}