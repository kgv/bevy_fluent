//! Resource asset

use super::{Error, Result};
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::tracing::instrument,
};
use fluent::FluentResource;
use std::{ops::Deref, str, sync::Arc};

/// [`FluentResource`](fluent::FluentResource) wrapper
#[derive(Asset, Clone, Debug, TypePath)]
pub struct ResourceAsset(pub(crate) Arc<FluentResource>);

impl Deref for ResourceAsset {
    type Target = FluentResource;

    fn deref(&self) -> &Self::Target {
        &self.0
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

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _: &'a Self::Settings,
        _: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut content = String::new();
        reader.read_to_string(&mut content).await?;
        Ok(ResourceAsset(deserialize(content)))
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}

#[instrument(skip_all)]
fn deserialize(content: String) -> Arc<FluentResource> {
    let fluent_resource = match FluentResource::try_new(content) {
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
    Arc::new(fluent_resource)
}
