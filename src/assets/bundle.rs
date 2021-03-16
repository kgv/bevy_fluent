use super::Resource;
use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use std::{ops::Deref, path::PathBuf, str};

async fn load_bundle<'a, 'b>(bytes: &'a [u8], load_context: &'a mut LoadContext<'b>) -> Result<()> {
    let paths = ron::de::from_bytes::<Vec<PathBuf>>(bytes)?;
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
    load_context
        .set_default_asset(LoadedAsset::new(Bundle(handles)).with_dependencies(asset_paths));
    Ok(())
}

/// `FluentBundle` wrapper.
///
/// Note: if locale fallback chain is empty then it is interlocale bundle.
///
/// # See Also
///
/// [`FluentBundle`](https://docs.rs/fluent/0.15.0/fluent/bundle/struct.FluentBundle.html).
#[derive(Clone, Debug, TypeUuid)]
#[uuid = "929113bb-9187-44c3-87be-6027fc3b7ac5"]
pub struct Bundle(pub(crate) Vec<Handle<Resource>>);

/// Bundle loader.
#[derive(Default)]
pub struct Loader;

impl AssetLoader for Loader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move { load_bundle(bytes, load_context).await })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

impl Deref for Bundle {
    type Target = Vec<Handle<Resource>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
