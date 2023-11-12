//! Resource asset

use super::{Error, Result};
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::{tracing::instrument, BoxedFuture},
};
use fluent::FluentResource;
use std::sync::Arc;

/// [`FluentResource`](fluent::FluentResource) wrapper
#[derive(Asset, Clone, Debug, Deref, TypePath, TypeUuid)]
#[uuid = "0b2367cb-fb4a-4746-a305-df98b26dddf6"]
pub struct ResourceAsset(pub(crate) Arc<FluentResource>);

impl ResourceAsset {
    pub fn new(resource: Arc<FluentResource>) -> Self {
        Self(resource)
    }
}

/// [`AssetLoader`](bevy::asset::AssetLoader) implementation for
/// [`ResourceAsset`]
#[derive(Default)]
pub struct ResourceAssetLoader;

impl AssetLoader for ResourceAssetLoader {
    type Asset = ResourceAsset;
    type Settings = ();
    type Error = Error;

    fn load<'a>(
        &self,
        reader: &'a mut Reader,
        _: &'a Self::Settings,
        _: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset>> {
        Box::pin(async move {
            let mut content = String::new();
            reader.read_to_string(&mut content).await?;
            Ok(ResourceAsset(Arc::new(deserialize(content))))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}

#[instrument(skip_all)]
fn deserialize(content: String) -> FluentResource {
    match FluentResource::try_new(content) {
        Ok(fluent_resource) => fluent_resource,
        Err((fluent_resource, errors)) => {
            error_span!("try_new").in_scope(|| {
                for error in errors {
                    error!(%error);
                }
            });
            fluent_resource
        }
    }
}
