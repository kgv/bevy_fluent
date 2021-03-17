use bevy::prelude::*;
use fluent::{bundle::FluentBundle, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use unic_langid::LanguageIdentifier;

/// Snapshot
///
/// Note: if locale fallback chain is empty then it is interlocale bundle.
pub struct Snapshot(
    HashMap<Option<LanguageIdentifier>, FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>,
);

impl Snapshot {
    pub fn locales(&self) -> impl Iterator<Item = Option<&LanguageIdentifier>> {
        self.keys().map(Option::as_ref)
    }
}

impl FromWorld for Snapshot {
    fn from_world(world: &mut World) -> Self {
        #[cfg(feature = "implicit")]
        let bundles = implicit::retrieve_bundles(world);
        #[cfg(not(feature = "implicit"))]
        let bundles = explicit::retrieve_bundles(world);
        debug!("`Snapshot` is initialized");
        Snapshot(bundles)
    }
}

impl Deref for Snapshot {
    type Target =
        HashMap<Option<LanguageIdentifier>, FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "implicit")]
mod implicit {
    use crate::{resources::Settings, FluentAsset};
    use bevy::{
        asset::{AssetPath, AssetServerSettings},
        prelude::*,
        utils::tracing::{self, instrument},
    };
    use fluent::{bundle::FluentBundle, FluentResource};
    use intl_memoizer::concurrent::IntlLangMemoizer;
    use std::{collections::HashMap, ffi::OsStr, path::Path, sync::Arc};
    use unic_langid::LanguageIdentifier;
    use walkdir::WalkDir;

    #[instrument(skip(world))]
    pub(super) fn retrieve_bundles(
        world: &mut World,
    ) -> HashMap<Option<LanguageIdentifier>, FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>
    {
        let AssetServerSettings { asset_folder } = world
            .get_resource::<AssetServerSettings>()
            .expect("get AssetServerSettings resource");
        let Settings { locales_folder, .. } = world
            .get_resource::<Settings>()
            .expect("get Settings resource");
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("get AssetServer resource");
        let fluent_assets = world
            .get_resource::<Assets<FluentAsset>>()
            .expect("get `Assets<Resource>` resource");
        let mut bundles = HashMap::new();
        for entry in WalkDir::new(Path::new(asset_folder).join(locales_folder)) {
            match entry {
                Ok(entry) => {
                    let mut path = entry.path();
                    if path.extension() == Some(OsStr::new("ftl")) {
                        path = path.strip_prefix(asset_folder).unwrap();
                        let asset_path = AssetPath::new(path.to_path_buf(), None);
                        let handle: Handle<FluentAsset> = asset_server.get_handle(asset_path);
                        path = path.strip_prefix(locales_folder).unwrap();
                        let locale = parse_locale(path);
                        let fluent_bundle = bundles.entry(locale.clone()).or_insert_with(|| {
                            FluentBundle::new_concurrent(locale.into_iter().collect())
                        });
                        if let Some(asset) = fluent_assets.get(handle) {
                            if let Err(errors) = fluent_bundle.add_resource(asset.0.clone()) {
                                warn_span!("add_resource").in_scope(|| {
                                    for error in errors {
                                        warn!(%error);
                                    }
                                });
                            }
                        }
                    }
                }
                Err(error) => error!(%error),
            }
        }
        bundles
    }

    fn parse_locale(path: &Path) -> Option<LanguageIdentifier> {
        // UNSTABLE: https://github.com/rust-lang/rust/issues/68537
        let mut language_identifiers = path
            .iter()
            .map(|component| {
                component
                    .to_str()
                    .map(|component| component.strip_suffix(".ftl").unwrap_or(component))?
                    .parse()
                    .ok()
            })
            .take_while(Option::is_some)
            .map(Option::unwrap);
        let mut parent: LanguageIdentifier = language_identifiers.next()?;
        for child in language_identifiers {
            if parent.matches(&child, true, false) {
                parent = child;
            } else {
                break;
            }
        }
        Some(parent)
    }
}

#[cfg(not(feature = "implicit"))]
mod explicit {
    use crate::{
        assets::{FluentAsset, LocaleAssets},
        resources::Settings,
    };
    use bevy::{
        asset::AssetServerSettings,
        prelude::*,
        utils::tracing::{self, instrument},
    };
    use fluent::{bundle::FluentBundle, FluentResource};
    use intl_memoizer::concurrent::IntlLangMemoizer;
    use std::{collections::HashMap, ffi::OsStr, path::Path, sync::Arc};
    use unic_langid::LanguageIdentifier;
    use walkdir::WalkDir;

    #[instrument(skip(world))]
    pub(super) fn retrieve_bundles(
        world: &mut World,
    ) -> HashMap<Option<LanguageIdentifier>, FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>
    {
        let AssetServerSettings { asset_folder } = world
            .get_resource::<AssetServerSettings>()
            .expect("get `AssetServerSettings` resource");
        let Settings {
            fallback_locale_chain,
            locales_folder,
            ..
        } = world
            .get_resource::<Settings>()
            .expect("get `Settings` resource");
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("get `AssetServer` resource");
        let fluent_assets = world
            .get_resource::<Assets<FluentAsset>>()
            .expect("get `Assets<Resource>` resource");
        let locale_assets = world
            .get_resource::<Assets<LocaleAssets>>()
            .expect("get `Assets<Bundle>` resource");
        let mut bundles = HashMap::new();
        for entry in WalkDir::new(Path::new(asset_folder).join(locales_folder)) {
            match entry {
                Ok(entry) => {
                    let mut path = entry.path();
                    if path.file_name() == Some(OsStr::new("locale.ron")) {
                        trace!("retrieve bundle: {:?}", entry);
                        path = path.strip_prefix(asset_folder).unwrap();
                        let handle: Handle<LocaleAssets> = asset_server.load(path);
                        path = path.strip_prefix(locales_folder).unwrap();
                        let locale = parse_locale(path);
                        if let Some(locale_assets) = locale_assets.get(handle) {
                            let bundle = build_bundle(fluent_assets, locale.clone(), locale_assets);
                            bundles.insert(locale, bundle);
                        }
                    }
                }
                Err(error) => error!(%error),
            }
        }
        bundles
    }

    #[instrument(skip(fluent_assets, locale_assets))]
    fn build_bundle(
        fluent_assets: &Assets<FluentAsset>,
        locale: Option<LanguageIdentifier>,
        locale_assets: &LocaleAssets,
    ) -> FluentBundle<Arc<FluentResource>, IntlLangMemoizer> {
        let mut fluent_bundle = FluentBundle::new_concurrent(locale.iter().cloned().collect());
        for handle in locale_assets.iter() {
            if let Some(fluent_asset) = fluent_assets.get(handle) {
                if let Err(errors) = fluent_bundle.add_resource(fluent_asset.0.clone()) {
                    warn_span!("add_resource").in_scope(|| {
                        for error in errors {
                            warn!(%error);
                        }
                    });
                }
            }
        }
        fluent_bundle
    }

    fn parse_locale(path: &Path) -> Option<LanguageIdentifier> {
        path.iter().rev().nth(1)?.to_str()?.parse().ok()
    }
}
