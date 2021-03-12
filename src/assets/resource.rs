use crate::Error;
use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use fluent::FluentResource;
use std::{
    ops::{Deref, DerefMut},
    str,
};

pub(crate) async fn load_fluent_resource<'a, 'b>(bytes: &'a [u8]) -> Result<FluentResource> {
    let source = str::from_utf8(bytes)?.to_string();
    let fluent_resource =
        FluentResource::try_new(source).map_err(|(_, errors)| Error::ParseResource(errors))?;
    Ok(fluent_resource)
}

async fn load_resource<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<()> {
    let fluent_resource = load_fluent_resource(bytes).await?;
    load_context.set_default_asset(LoadedAsset::new(Resource(fluent_resource)));
    Ok(())
}

/// `FluentResource` wrapper.
///
/// # See Also
///
/// [`FluentResource`](https://docs.rs/fluent/0.15.0/fluent/struct.FluentResource.html).
#[derive(Debug, TypeUuid)]
#[uuid = "0b2367cb-fb4a-4746-a305-df98b26dddf6"]
pub struct Resource(FluentResource);

impl Deref for Resource {
    type Target = FluentResource;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Resource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Resource loader.
#[derive(Default)]
pub struct Loader;

impl AssetLoader for Loader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move { load_resource(bytes, load_context).await })
    }

    fn extensions(&self) -> &[&str] {
        &["fluent", "ftl"]
    }
}
