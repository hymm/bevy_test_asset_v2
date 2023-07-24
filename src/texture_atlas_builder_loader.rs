use bevy::asset::processor::{AssetProcessor, LoadAndSave};
use bevy::asset::{saver::AssetSaver, AssetLoader};
use bevy::asset::{AsyncReadExt, AsyncWriteExt};
use bevy::prelude::{Asset, AssetApp, Image, Plugin, Rect, Vec2};
use bevy::reflect::TypePath;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::TextureAtlas;
use serde::{Deserialize, Serialize};

pub struct TextureAtlasBuilderPlugin;
impl Plugin for TextureAtlasBuilderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_asset_loader(TextureAtlasBuilderLoader)
            .register_asset_loader(BuiltAtlasLoader);

        if let Some(processor) = app.world.get_resource::<AssetProcessor>() {
            processor.register_processor::<LoadAndSave<TextureAtlasBuilderLoader, BuiltAtlasSaver>>(
                BuiltAtlasSaver.into(),
            );
        }
    }
}

/// loads a texture atlas from a config file
struct TextureAtlasBuilderLoader;

#[derive(Serialize, Deserialize, Default)]
struct TextureAtlasConfigLoaderSettings {}

#[derive(Asset, TypePath, Serialize, Deserialize, Default)]
struct TextureAtlasBuilder {
    source_images: Vec<String>,
}

impl AssetLoader for TextureAtlasBuilderLoader {
    type Asset = TextureAtlas;

    type Settings = ();

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ron: TextureAtlasBuilder = ron::de::from_bytes(&bytes)?;

            let mut builder = crate::texture_atlas_builder::TextureAtlasBuilder::default();

            for src in ron.source_images {
                let loaded = load_context.load_direct(&src).await?;
                let image = loaded.take::<Image>().unwrap();
                builder.add_texture(src, image);
            }

            let Ok((image, areas)) = builder.finish() else { panic!("blah!") };

            let dimensions = image.size();
            let image_handle = load_context.add_labeled_asset("image".into(), image);

            let mut texture_atlas = TextureAtlas::new_empty(image_handle, dimensions);

            for area in areas {
                texture_atlas.add_texture(area);
            }

            // let atlas_handle = load_context.add_labeled_asset("atlas".into(), texture_atlas);

            Ok(texture_atlas)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["atlas_builder"]
    }
}

// #[derive(Asset, TypePath)]
// struct BuiltAtlas {
//     pub atlas_handle: Handle<TextureAtlas>,
// }

struct BuiltAtlasSaver;

impl AssetSaver for BuiltAtlasSaver {
    type Asset = TextureAtlas;
    type Settings = ();
    type OutputLoader = BuiltAtlasLoader;

    fn save<'a>(
        &'a self,
        writer: &'a mut bevy::asset::io::Writer,
        asset: bevy::asset::saver::SavedAsset<'a, Self::Asset>,
        _settings: &'a Self::Settings,
    ) -> bevy::utils::BoxedFuture<
        'a,
        Result<<Self::OutputLoader as AssetLoader>::Settings, anyhow::Error>,
    > {
        Box::pin(async move {
            // TODO: convert unwrap into an error
            // let texture_atlas = asset.get_labeled::<TextureAtlas>("atlas").unwrap();
            let image = asset.get_labeled::<Image>("image").unwrap();

            let saved_atlas = SavedAtlas {
                areas: asset.get().textures.clone(),
                image_size: image.get().size(),
                image_bytes: image.get().data.clone(),
            };

            let text = ron::to_string(&saved_atlas)?;
            writer.write_all(text.as_bytes()).await?;

            Ok(())
        })
    }
}

#[derive(Asset, TypePath, Serialize, Deserialize)]
struct SavedAtlas {
    areas: Vec<Rect>,
    image_size: Vec2,
    image_bytes: Vec<u8>,
}

struct BuiltAtlasLoader;

impl AssetLoader for BuiltAtlasLoader {
    type Asset = TextureAtlas;

    type Settings = ();

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let saved_atlas: SavedAtlas = ron::de::from_bytes(&bytes)?;

            let image = Image::new(
                Extent3d {
                    width: saved_atlas.image_size.x as u32,
                    height: saved_atlas.image_size.y as u32,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                saved_atlas.image_bytes.clone(),
                TextureFormat::Rgba8Unorm,
            );

            let image_handle = load_context.add_labeled_asset("image".into(), image);

            let mut texture_atlas = TextureAtlas::new_empty(image_handle, saved_atlas.image_size);
            for area in &saved_atlas.areas {
                texture_atlas.add_texture(*area);
            }
            // let texture_atlas_handle =
            //     load_context.add_labeled_asset("atlas".into(), texture_atlas);

            Ok(texture_atlas)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["atlas_builder"]
    }
}
