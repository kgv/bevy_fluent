use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::{
    assets::bundles::{Locale, Settings},
    prelude::*,
};
use fluent_content::Content;
use unic_langid::langid;

// Single locale.
pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "examples/default_locale/assets".to_string(),
                ..default()
            }),
            FluentPlugin,
        ))
        .add_systems(
            Update,
            (
                // implicit_default_locale,
                // explicit_default_locale,
                no_default_locale,
            ),
        )
        .run();
}

// Loads a bundle with implicit default locale (without settings).
fn implicit_default_locale(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundlesAsset>>,
    mut handle: Local<Option<Handle<BundlesAsset>>>,
) {
    let handle = &*handle.get_or_insert_with(|| asset_server.load("locales/.ftl.ron"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].locales, ["en-US"]);
        assert_eq!(bundles.content("hello-world").unwrap(), "hello world");
    }
}

// Loads a bundle with explicit default locale.
fn explicit_default_locale(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundlesAsset>>,
    mut handle: Local<Option<Handle<BundlesAsset>>>,
) {
    let handle = &*handle.get_or_insert_with(|| {
        asset_server.load_with_settings("locales/.ftl.ron", |settings: &mut Settings| {
            settings.locales = vec![Locale {
                requested: vec![langid!("ru-RU")],
                default: DefaultLocale(Some(langid!("de-DE"))),
                ..default()
            }];
        })
    });
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].locales, ["ru-RU", "de-DE"]);
        assert_eq!(bundles.content("hello-world").unwrap(), "привет мир");
    }
}

// Loads a bundle with no default locale.
fn no_default_locale(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundlesAsset>>,
    mut handle: Local<Option<Handle<BundlesAsset>>>,
) {
    let handle = &*handle.get_or_insert_with(|| {
        asset_server.load_with_settings("locales/.ftl.ron", |settings: &mut Settings| {
            settings.locales = vec![Locale {
                requested: vec![langid!("ru-RU")],
                default: DefaultLocale(None),
                ..default()
            }];
        })
    });
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].locales, ["ru-RU"]);
        assert_eq!(bundles.content("hello-world").unwrap(), "привет мир");
    }
}
