use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::{
    assets::bundles::{Locale, Settings},
    prelude::*,
};
use fluent_content::Content;
use unic_langid::langid;

// Locale fallback chain.
pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "examples/complex/assets".to_string(),
                ..default()
            }),
            FluentPlugin,
        ))
        .add_systems(Update, (monolingual, multilingual))
        .run();
}

// Loads locales fallback chain bundle (monolingual) by settings.
// - Note: the `BundlesAsset` file uses the yaml format.
fn monolingual(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundlesAsset>>,
    mut handle: Local<Option<Handle<BundlesAsset>>>,
) {
    let handle = &*handle.get_or_insert_with(|| {
        asset_server.load_with_settings(
            "locales/monolingual/.ftl.yml",
            |settings: &mut Settings| {
                settings.locales.push(Locale {
                    requested: vec![langid!("ru")],
                    default: Some(langid!("en-US")),
                    ..default()
                });
            },
        )
    });
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles["ru"].locales, ["ru-RU", "ru-BY", "en-US"]);
        // From ru-RU bundle, the first in fallback chain.
        assert_eq!(bundles.content("hello").unwrap(), "привет");
        // From ru-BY bundle, the second in fallback chain.
        assert_eq!(bundles.content("world").unwrap(), "свету");
        // From en-US bundle, the last in fallback chain, default locale.
        assert_eq!(bundles.content("bevy").unwrap(), "bevy");
    }
}

// Loads locales fallback chain bundles (multilingual) by settings.
// - Note: the `BundlesAsset` file contains default locale selector ("de-DE").
//   But the locale "ru" overrides it to "en-US" via settings.
fn multilingual(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundlesAsset>>,
    mut handle: Local<Option<Handle<BundlesAsset>>>,
) {
    let handle = &*handle.get_or_insert_with(|| {
        asset_server.load_with_settings(
            "locales/multilingual/.ftl.ron",
            |settings: &mut Settings| {
                settings.locales = vec![
                    Locale {
                        requested: vec![langid!("de")],
                        ..default()
                    },
                    Locale {
                        requested: vec![langid!("en")],
                        ..default()
                    },
                    Locale {
                        requested: vec![langid!("ru")],
                        default: Some(langid!("en-US")),
                        ..default()
                    },
                ];
            },
        )
    });
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 3);
        assert_eq!(bundles["de"].locales, ["de-DE"]);
        assert_eq!(bundles["en"].locales, ["en-US", "de-DE"]);
        assert_eq!(bundles["ru"].locales, ["ru-RU", "en-US"]);
        // From de-DE bundle.
        assert_eq!(bundles["de"].content("hello-world").unwrap(), "hallo welt");
        // From en-US bundle.
        assert_eq!(bundles["en"].content("hello-world").unwrap(), "hello world");
        // From ru-RU bundle.
        assert_eq!(bundles["ru"].content("hello-world").unwrap(), "привет мир");
    }
}
