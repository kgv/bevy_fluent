//! Resource asset

use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::{
        tracing::{self, instrument},
        BoxedFuture,
    },
};
use fluent::FluentResource;
use std::{ops::Deref, str, sync::Arc};

#[instrument(skip_all)]
pub(crate) fn deserialize(bytes: &[u8]) -> Result<Arc<FluentResource>> {
    let string = str::from_utf8(bytes)?.to_string();
    let fluent_resource = match FluentResource::try_new(string) {
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
    Ok(Arc::new(fluent_resource))
}

#[instrument(fields(path = %load_context.path().display()), skip_all)]
fn load(data: Arc<FluentResource>, load_context: &mut LoadContext<'_>) {
    load_context.set_default_asset(LoadedAsset::new(ResourceAsset(data)));
}

/// [`FluentResource`](fluent::FluentResource) wrapper
#[derive(Clone, Debug, TypeUuid)]
#[uuid = "0b2367cb-fb4a-4746-a305-df98b26dddf6"]
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
    fn load<'a>(
        &self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            load(deserialize(bytes)?, load_context);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}
