//! Bundle asset

use super::{Error, Result};
use crate::ResourceAsset;
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::{tracing::instrument, BoxedFuture},
};
use fluent::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::PathBuf, str, sync::Arc};
use unic_langid::LanguageIdentifier;

/// [`FluentBundle`](fluent::bundle::FluentBundle) wrapper
///
/// Collection of [`FluentResource`]s for a single locale
#[derive(Asset, Clone, TypePath)]
pub struct BundleAsset(pub(crate) Arc<FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>);

impl Deref for BundleAsset {
    type Target = FluentBundle<Arc<FluentResource>, IntlLangMemoizer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`AssetLoader`](bevy::asset::AssetLoader) implementation for [`BundleAsset`]
#[derive(Default)]
pub struct BundleAssetLoader;

impl AssetLoader for BundleAssetLoader {
    type Asset = BundleAsset;
    type Settings = ();
    type Error = Error;

    fn load<'a>(
        &self,
        reader: &'a mut Reader,
        _: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset>> {
        Box::pin(async move {
            let path = load_context.path();
            let mut content = String::new();
            reader.read_to_string(&mut content).await?;
            match path.extension() {
                Some(extension) if extension == "ron" => {
                    load(ron::de::from_str(&content)?, load_context).await
                }
                Some(extension) if extension == "yaml" || extension == "yml" => {
                    load(serde_yaml::from_str(&content)?, load_context).await
                }
                _ => unreachable!("We already check all the supported extensions."),
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl.ron", "ftl.yaml", "ftl.yml"]
    }
}

/// Data
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Data {
    locale: LanguageIdentifier,
    resources: Vec<PathBuf>,
}

#[instrument(fields(path = %load_context.path().display()), skip_all)]
async fn load(data: Data, load_context: &mut LoadContext<'_>) -> Result<BundleAsset> {
    let mut bundle = FluentBundle::new_concurrent(vec![data.locale.clone()]);
    for mut path in data.resources {
        if path.is_relative() {
            if let Some(parent) = load_context.path().parent() {
                path = parent.join(path);
            }
        }
        let loaded = load_context.load_direct(path).await?;
        let resource = loaded.get::<ResourceAsset>().unwrap();
        if let Err(errors) = bundle.add_resource(resource.0.clone()) {
            warn_span!("add_resource").in_scope(|| {
                for error in errors {
                    warn!(%error);
                }
            });
        }
    }
    Ok(BundleAsset(Arc::new(bundle)))
}
