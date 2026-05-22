//! Bundle asset

use super::{Error, Result};
use crate::ResourceAsset;
use bevy::{
    asset::{io::Reader, AssetLoader, AssetPath, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use fluent::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::PathBuf, str, sync::Arc};
use tracing::instrument;
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
#[derive(Default, TypePath)]
pub struct BundleAssetLoader;

impl AssetLoader for BundleAssetLoader {
    type Asset = BundleAsset;
    type Settings = ();
    type Error = Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let path = load_context.path().path();
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

#[instrument(fields(path = %load_context.path().path().display()), skip_all)]
async fn load(data: Data, load_context: &mut LoadContext<'_>) -> Result<BundleAsset> {
    let mut bundle = FluentBundle::new_concurrent(vec![data.locale.clone()]);
    let base = load_context.path().clone();
    for path in data.resources {
        let path_str = path.to_string_lossy();
        let resolved: AssetPath<'static> = if path.is_relative() {
            base.resolve_embed(&path_str)?.into_owned()
        } else {
            AssetPath::parse(&path_str).clone_owned()
        };
        let loaded = load_context
            .loader()
            .immediate()
            .with_unknown_type()
            .load(resolved)
            .await?;
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
