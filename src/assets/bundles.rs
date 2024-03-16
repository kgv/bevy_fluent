//! Bundle asset

use super::{Error, Result};
use crate::{resources::FallbackChain, ResourceAsset};
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use fluent_content::{Content, Request};
use indexmap::{indexmap, IndexMap};
use intl_memoizer::concurrent::IntlLangMemoizer;
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    borrow::Borrow,
    collections::HashMap,
    ffi::OsStr,
    fmt::{self, Debug, Display, Formatter},
    path::PathBuf,
    sync::Arc,
};
use tracing::instrument;
use unic_langid::LanguageIdentifier;

type Bundle = FluentBundle<Arc<FluentResource>, IntlLangMemoizer>;

/// Collection of [`FluentBundle`]s
#[derive(Asset, Default, Deref, TypePath)]
pub struct BundlesAsset(pub IndexMap<String, Bundle>);

impl<'a, T, U> Content<'a, T, U> for BundlesAsset
where
    T: Copy + Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>>,
{
    #[instrument(fields(request = %request.into()), skip_all)]
    fn content(&self, request: T) -> Option<String> {
        self.0.values().find_map(|bundle| {
            let content = bundle.content(request);
            trace!(locale = %bundle.locales[0], ?content);
            content
        })
    }
}

#[derive(Default)]
pub struct BundlesAssetLoader;

impl AssetLoader for BundlesAssetLoader {
    type Asset = BundlesAsset;
    type Settings = Settings;
    type Error = Error;

    fn load<'a>(
        &self,
        reader: &'a mut Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset>> {
        Box::pin(async move {
            let mut content = String::new();
            reader.read_to_string(&mut content).await?;
            let deserialized: Deserialized =
                match load_context.path().extension().and_then(OsStr::to_str) {
                    Some("ron") => ron::de::from_str(&content)?,
                    Some("yml" | "yaml") => serde_yaml::from_str(&content)?,
                    _ => unreachable!("We already check all the supported extensions."),
                };
            debug!(?deserialized);
            debug!(default = ?deserialized.default.as_ref().map(ToString::to_string));
            // Labels
            for (locale, paths) in &deserialized.bundles {
                load_context.add_loaded_labeled_asset(locale.to_string(), {
                    let mut load_context = load_context.begin_labeled_asset();
                    let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
                    bundle.load(locale, paths, &mut load_context).await?;
                    load_context.finish(BundlesAsset(indexmap![locale.to_string() => bundle]), None)
                });
            }
            // Settings
            debug!(?settings);
            let mut bundles = IndexMap::new();
            for (index, locales) in settings.locales.iter().enumerate() {
                if let Some(locale) = &locales.default {
                    if !deserialized.bundles.contains_key(locale) {
                        return Err(Error::LocaleNotFound {
                            locale: locale.clone(),
                            path: load_context.path().to_path_buf(),
                        });
                    }
                }
                let key = locales
                    .name
                    .clone()
                    .or(locales.requested.first().map(ToString::to_string))
                    .unwrap_or(index.to_string());
                let mut fallback_chain = FallbackChain::new(deserialized.bundles.keys());
                fallback_chain.default = locales.default.as_ref().or(deserialized.default.as_ref());
                let locales = fallback_chain.request(&locales.requested);
                info!(locales = ?locales.iter().map(ToString::to_string).collect::<Vec<_>>());
                let mut bundle = FluentBundle::new_concurrent(
                    locales.iter().copied().copied().cloned().collect(),
                );
                for &&locale in &locales {
                    let paths = &deserialized.bundles[locale];
                    bundle.load(locale, paths, load_context).await?;
                }
                bundles.insert(key, bundle);
            }
            if bundles.is_empty() {
                if let Some(locale) = &deserialized.default {
                    info!(%locale);
                    let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
                    let paths = &deserialized.bundles[locale];
                    bundle.load(locale, paths, load_context).await?;
                    bundles.insert(locale.to_string(), bundle);
                }
            }
            Ok(BundlesAsset(bundles))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl.ron", "ftl.yaml", "ftl.yml"]
    }
}

/// Fluent bundle loader
trait FluentBundleLoader {
    async fn load(
        &mut self,
        locale: &LanguageIdentifier,
        paths: &[PathBuf],
        load_context: &mut LoadContext<'_>,
    ) -> Result<()>;
}

impl FluentBundleLoader for Bundle {
    async fn load(
        &mut self,
        locale: &LanguageIdentifier,
        paths: &[PathBuf],
        load_context: &mut LoadContext<'_>,
    ) -> Result<()> {
        for path in paths {
            let mut path = path.clone();
            if path.is_relative() {
                if let Some(parent) = load_context.path().parent() {
                    path = parent.join(&path);
                }
            }
            let loaded = load_context.load_direct(path).await?;
            let resource = loaded.get::<ResourceAsset>().unwrap();
            if let Err(errors) = self.add_resource(resource.0.clone()) {
                warn_span!("add_resource").in_scope(|| {
                    for error in errors {
                        warn!(%locale, %error);
                    }
                });
            }
        }
        Ok(())
    }
}

/// Settings
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Settings {
    pub locales: Vec<Locale>,
}

// impl Display for Settings {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         f.debug_struct("Settings")
//             .field("locales", &format_args!(r#""{}""#, &self.locales))
//             .finish()
//     }
// }

/// Locale settings
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Locale {
    /// Locale name
    pub name: Option<String>,
    /// Requested locales
    pub requested: Vec<LanguageIdentifier>,
    /// Default locale
    pub default: Option<LanguageIdentifier>,
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut requested = self.requested.iter().peekable();
        while let Some(locale) = requested.next() {
            write!(f, "{locale}")?;
            if self.default.is_some() || requested.peek().is_some() {
                f.write_str("|")?;
            }
        }
        if let Some(locale) = &self.default {
            write!(f, "*{locale}")?;
        }
        Ok(())
    }
}

/// Deserialized
#[derive(Clone, Debug, Default, Serialize)]
struct Deserialized {
    bundles: HashMap<LanguageIdentifier, Vec<PathBuf>>,
    default: Option<LanguageIdentifier>,
}

impl<'de> Deserialize<'de> for Deserialized {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let deserialized: HashMap<Key, _> = HashMap::deserialize(deserializer)?;
        let mut default = None;
        let bundles = deserialized
            .into_iter()
            .map(|(key, value)| match key {
                Key::Default(key) => {
                    default = Some(key.clone());
                    (key, value)
                }
                Key::NonDefault(key) => (key, value),
            })
            .collect();
        Ok(Self { bundles, default })
    }
}

/// Key
#[derive(Eq, Hash, PartialEq)]
enum Key {
    Default(LanguageIdentifier),
    NonDefault(LanguageIdentifier),
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let deserialized = String::deserialize(deserializer)?;
        Ok(if let Some(stripped) = deserialized.strip_prefix('*') {
            Self::Default(stripped.parse().map_err(serde::de::Error::custom)?)
        } else {
            Self::NonDefault(deserialized.parse().map_err(serde::de::Error::custom)?)
        })
    }
}

// #[cfg(test)]
// mod test {
//     use unic_langid::langid;

//     #[test]
//     fn test() {
//         let label = super::Locale {
//             default: None,
//             requested: vec![langid!("en-US"), langid!("ru-RU")],
//         };
//         assert_eq!(label.to_string(), "/en-US/ru-RU");
//         let label = super::Locale {
//             default: Some(langid!("de-DE")),
//             requested: vec![langid!("en-US"), langid!("ru-RU")],
//         };
//         assert_eq!(label.to_string(), "de-DE/en-US/ru-RU");
//     }
// }
