use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use fluent::FluentResource;
use std::{ops::Deref, str, sync::Arc};

async fn load_asset<'a, 'b>(bytes: &'a [u8], load_context: &'a mut LoadContext<'b>) -> Result<()> {
    let source = str::from_utf8(bytes)?.to_string();
    let fluent_resource = match FluentResource::try_new(source) {
        Ok(fluent_resource) => fluent_resource,
        Err((fluent_resource, errors)) => {
            error_span!("try_new").in_scope(|| {
                for error in errors {
                    error!(%error);
                }
            });
            fluent_resource
        }
    };
    load_context.set_default_asset(LoadedAsset::new(FluentAsset(Arc::new(fluent_resource))));
    Ok(())
}

/// `FluentResource` wrapper
///
/// # See Also
///
/// [`FluentResource`](https://docs.rs/fluent/0.15.0/fluent/struct.FluentResource.html).
#[derive(Clone, Debug, TypeUuid)]
#[uuid = "0b2367cb-fb4a-4746-a305-df98b26dddf6"]
pub struct FluentAsset(pub(crate) Arc<FluentResource>);

impl Deref for FluentAsset {
    type Target = FluentResource;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`AssetLoader`](bevy::asset::AssetLoader) implementation for [`FluentAsset`]
#[derive(Default)]
pub struct FluentAssetLoader;

impl AssetLoader for FluentAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move { load_asset(bytes, load_context).await })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}
