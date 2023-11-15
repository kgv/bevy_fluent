use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::prelude::*;
use fluent_content::Content;

// Single locale.
pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "examples/trivial/assets".to_string(),
                ..default()
            }),
            FluentPlugin,
        ))
        .add_systems(Update, (empty, selector, label))
        .run();
}

// Loads a bundle without everything.
// - Note: no default locale selector is set, no label is set, no settings are
//   set. In this case `BundlesAsset` will be empty.
fn empty(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundlesAsset>>,
    mut handle: Local<Option<Handle<BundlesAsset>>>,
) {
    let handle =
        &*handle.get_or_insert_with(|| asset_server.load("locales/without_default.ftl.ron"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 0);
    }
}

// Loads a bundle using a default locale selector.
// - Note: the default locale selector is set ("en-US").
fn selector(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundlesAsset>>,
    mut handle: Local<Option<Handle<BundlesAsset>>>,
) {
    let handle = &*handle.get_or_insert_with(|| asset_server.load("locales/with_default.ftl.ron"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].locales, ["en-US"]);
        assert_eq!(bundles.content("hello-world").unwrap(), "hello world");
    }
}

// Loads a bundle using a label.
// - Note: whether a default locale selector is set or not, the bandle will be
//   loaded by label.
fn label(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundlesAsset>>,
    mut ru: Local<Option<Handle<BundlesAsset>>>,
    mut en: Local<Option<Handle<BundlesAsset>>>,
) {
    let handle =
        &*en.get_or_insert_with(|| asset_server.load("locales/without_default.ftl.ron#en-US"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].locales, ["en-US"]);
        assert_eq!(bundles.content("hello-world").unwrap(), "hello world");
    }
    let handle =
        &*ru.get_or_insert_with(|| asset_server.load("locales/with_default.ftl.ron#ru-RU"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundles = assets.get(handle).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].locales, ["ru-RU"]);
        assert_eq!(bundles.content("hello-world").unwrap(), "привет мир");
    }
}
