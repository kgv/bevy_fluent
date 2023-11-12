//! Bundle asset

use super::{Error, Result};
use crate::ResourceAsset;
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};
use fluent::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf, sync::Arc};
use unic_langid::LanguageIdentifier;

/// [`FluentBundle`](fluent::bundle::FluentBundle) wrapper
///
/// Collection of [`FluentResource`]s for a single locale
#[derive(Asset, Clone, Deref, TypePath, TypeUuid)]
#[uuid = "929113bb-9187-44c3-87be-6027fc3b7ac5"]
pub struct BundleAsset(pub(crate) Arc<FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>);

impl BundleAsset {
    pub fn new(bundle: Arc<FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>) -> Self {
        Self(bundle)
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
            let mut content = String::new();
            reader.read_to_string(&mut content).await?;
            let resources: Resources = match load_context.path().extension().and_then(OsStr::to_str)
            {
                Some("ron") => ron::de::from_str(&content)?,
                Some("yml" | "yaml") => serde_yaml::from_str(&content)?,
                _ => unreachable!("We already check all the supported extensions."),
            };
            for (locale, paths) in resources {
                load_context.add_loaded_labeled_asset(locale.to_string(), {
                    let mut load_context = load_context.begin_labeled_asset();
                    let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
                    for mut path in paths {
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
                                    warn!(%locale, %error);
                                }
                            });
                        }
                    }
                    load_context.finish(BundleAsset(Arc::new(bundle)), None)
                });
            }
            Ok(BundleAsset(Arc::new(FluentBundle::new_concurrent(vec![]))))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl.ron", "ftl.yaml", "ftl.yml"]
    }
}

/// Resources
type Resources = HashMap<LanguageIdentifier, Vec<PathBuf>>;
