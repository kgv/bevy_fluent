use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::prelude::*;
use fluent_content::Content;

// Single locale.
pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "examples/labels/assets".to_string(),
                ..default()
            }),
            FluentPlugin,
        ))
        .add_systems(Update, labels)
        .run();
}

// Loads a bundle with a label.
fn labels(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundlesAsset>>,
    mut en: Local<Option<Handle<BundlesAsset>>>,
    mut de: Local<Option<Handle<BundlesAsset>>>,
    mut ru: Local<Option<Handle<BundlesAsset>>>,
) {
    let handle = &*en.get_or_insert_with(|| asset_server.load("locales/.ftl.ron#en-US"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].locales, ["en-US"]);
        assert_eq!(bundles.content("hello-world").unwrap(), "hello world");
    }
    let handle = &*de.get_or_insert_with(|| asset_server.load("locales/.ftl.ron#de-DE"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].locales, ["de-DE"]);
        assert_eq!(bundles.content("hello-world").unwrap(), "hallo welt");
    }
    let handle = &*ru.get_or_insert_with(|| asset_server.load("locales/.ftl.ron#ru-RU"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].locales, ["ru-RU"]);
        assert_eq!(bundles.content("hello-world").unwrap(), "привет мир");
    }
}
