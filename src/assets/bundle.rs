//! Bundle asset module

use super::ResourceAsset;
use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;
use std::{path::PathBuf, str};
use unic_langid::LanguageIdentifier;

async fn load_asset<'a, 'b>(bytes: &'a [u8], load_context: &'a mut LoadContext<'b>) -> Result<()> {
    let Intermediate {
        locale,
        resources: paths,
    } = ron::de::from_bytes(bytes)?;
    let mut handles = Vec::new();
    let mut asset_paths = Vec::new();
    let parent = load_context.path().parent().unwrap();
    for mut path in paths {
        if path.is_relative() {
            path = parent.join(path);
        }
        let asset_path = AssetPath::new(path, None);
        asset_paths.push(asset_path.clone());
        let handle = load_context.get_handle(asset_path);
        handles.push(handle);
    }
    // Add child assets as dependencies to make sure it is loaded by the asset
    // server when our bundle is.
    load_context.set_default_asset(
        LoadedAsset::new(BundleAsset {
            locale,
            resources: handles,
        })
        .with_dependencies(asset_paths),
    );
    Ok(())
}

/// [`FluentBundle`](fluent::bundle::FluentBundle) wrapper
///
/// Collection of [`ResourceAsset`]'s handles for a single locale
#[derive(Clone, Debug, TypeUuid)]
#[uuid = "929113bb-9187-44c3-87be-6027fc3b7ac5"]
pub struct BundleAsset {
    locale: Option<LanguageIdentifier>,
    resources: Vec<Handle<ResourceAsset>>,
}

impl BundleAsset {
    pub fn locale(&self) -> Option<&LanguageIdentifier> {
        self.locale.as_ref()
    }

    pub fn resources(&self) -> &[Handle<ResourceAsset>] {
        &self.resources
    }
}

/// [`AssetLoader`](bevy::asset::AssetLoader) implementation for [`BundleAsset`]
#[derive(Default)]
pub struct BundleAssetLoader;

impl AssetLoader for BundleAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move { load_asset(bytes, load_context).await })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[derive(Debug, Deserialize)]
struct Intermediate {
    locale: Option<LanguageIdentifier>,
    resources: Vec<PathBuf>,
}
