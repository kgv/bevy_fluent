use bevy::prelude::*;
use fluent::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use unic_langid::LanguageIdentifier;

/// Snapshot.
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
        let bundles = implicit::bundles(world);
        #[cfg(not(feature = "implicit"))]
        let bundles = explicit::bundles(world);
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
    use crate::{assets::Resource, resources::Settings};
    use bevy::{
        asset::{AssetPath, AssetServerSettings},
        prelude::*,
    };
    use fluent::{bundle::FluentBundle, FluentResource};
    use intl_memoizer::concurrent::IntlLangMemoizer;
    use std::{collections::HashMap, ffi::OsStr, path::Path, sync::Arc};
    use unic_langid::LanguageIdentifier;
    use walkdir::WalkDir;

    pub(super) fn bundles(
        world: &mut World,
    ) -> HashMap<Option<LanguageIdentifier>, FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>
    {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("get AssetServer resource");
        let asset_server_settings = world
            .get_resource::<AssetServerSettings>()
            .expect("get AssetServerSettings resource");
        let settings = world
            .get_resource::<Settings>()
            .expect("get Settings resource");
        let resource_assets = world
            .get_resource::<Assets<Resource>>()
            .expect("get `Assets<Resource>` resource");

        retrieve_resources(
            &asset_server,
            &asset_server_settings.asset_folder,
            &settings.locale_folder,
            &resource_assets,
        )
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

    fn retrieve_resources(
        asset_server: &AssetServer,
        asset_folder: &str,
        locale_folder: &str,
        resource_assets: &Assets<Resource>,
    ) -> HashMap<Option<LanguageIdentifier>, FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>
    {
        let mut fluent_bundles = HashMap::new();
        for entry in WalkDir::new(Path::new(asset_folder).join(locale_folder)) {
            match entry {
                Ok(entry) => {
                    let mut path = entry.path();
                    if path.extension() == Some(OsStr::new("ftl")) {
                        path = path.strip_prefix(asset_folder).unwrap();
                        let asset_path = AssetPath::new(path.to_path_buf(), None);
                        let handle: Handle<Resource> = asset_server.get_handle(asset_path);
                        path = path.strip_prefix(locale_folder).unwrap();
                        let locale = parse_locale(path);
                        let fluent_bundle =
                            fluent_bundles.entry(locale.clone()).or_insert_with(|| {
                                FluentBundle::new_concurrent(locale.into_iter().collect())
                            });

                        if let Some(resource) = resource_assets.get(handle) {
                            if let Err(errors) = fluent_bundle.add_resource(resource.0.clone()) {
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
        fluent_bundles
    }
}

#[cfg(not(feature = "implicit"))]
mod explicit {
    use crate::{
        assets::{Bundle, Resource},
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

    #[instrument(level = "debug", skip(world))]
    pub(super) fn bundles(
        world: &mut World,
    ) -> HashMap<Option<LanguageIdentifier>, FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>
    {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("get `AssetServer` resource");
        let asset_server_settings = world
            .get_resource::<AssetServerSettings>()
            .expect("get `AssetServerSettings` resource");
        let bundle_assets = world
            .get_resource::<Assets<Bundle>>()
            .expect("get `Assets<Bundle>` resource");
        let resource_assets = world
            .get_resource::<Assets<Resource>>()
            .expect("get `Assets<Resource>` resource");
        let settings = world
            .get_resource::<Settings>()
            .expect("get `Settings` resource");

        let bundles = retrieve_bundles(
            asset_server,
            &asset_server_settings.asset_folder,
            &settings.locale_folder,
        );
        let mut fluent_bundles = HashMap::new();
        for (locale, handle) in bundles.iter() {
            if let Some(bundle) = bundle_assets.get(handle) {
                let mut fluent_bundle =
                    FluentBundle::new_concurrent(locale.iter().cloned().collect());
                for handle in &bundle.0 {
                    if let Some(resource) = resource_assets.get(handle) {
                        if let Err(errors) = fluent_bundle.add_resource(resource.0.clone()) {
                            warn_span!("add_resource").in_scope(|| {
                                for error in errors {
                                    warn!(%error);
                                }
                            });
                        }
                    }
                }
                fluent_bundles.insert(locale.clone(), fluent_bundle);
            }
        }
        fluent_bundles
    }

    fn parse_locale(path: &Path) -> Option<LanguageIdentifier> {
        path.iter().rev().nth(1)?.to_str()?.parse().ok()
    }

    #[instrument(level = "debug", skip(asset_server))]
    fn retrieve_bundles(
        asset_server: &AssetServer,
        asset_folder: &str,
        locale_folder: &str,
    ) -> HashMap<Option<LanguageIdentifier>, Handle<Bundle>> {
        let mut bundles = HashMap::new();
        for entry in WalkDir::new(Path::new(asset_folder).join(locale_folder)) {
            match entry {
                Ok(entry) => {
                    let mut path = entry.path();
                    if path.file_name() == Some(OsStr::new("locale.ron")) {
                        trace!("retrieve bundle: {:?}", entry);
                        // Hierarchy starts with the
                        // `asset_folder`/`locale_folder` directory.
                        // So we can safe strip `asset_folder` at first
                        path = path.strip_prefix(asset_folder).unwrap();
                        let handle = asset_server.load(path);
                        // And then safe strip `locale_folder`.
                        path = path.strip_prefix(locale_folder).unwrap();
                        let locale = parse_locale(path);
                        bundles.insert(locale, handle);
                    }
                }
                Err(error) => error!(%error),
            }
        }
        bundles
    }
}
