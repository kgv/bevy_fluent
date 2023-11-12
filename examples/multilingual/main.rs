use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::prelude::*;
use fluent_content::Content;

pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "examples/multilingual/assets".to_string(),
                ..default()
            }),
            FluentPlugin,
        ))
        .add_systems(Update, localized_hello_world)
        .run();
}

fn localized_hello_world(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundleAsset>>,
    mut de: Local<Option<Handle<BundleAsset>>>,
    mut en: Local<Option<Handle<BundleAsset>>>,
    mut ru: Local<Option<Handle<BundleAsset>>>,
) {
    let de = &*de.get_or_insert_with(|| asset_server.load("locales/.ftl.ron#de-DE"));
    if asset_server.get_load_state(de) == Some(LoadState::Loaded) {
        let de = assets.get(de).unwrap();
        // From de-DE bundle.
        assert!(matches!(de.content("hello-world"), Some(content) if content == "hallo welt"));
    }
    let en = &*en.get_or_insert_with(|| asset_server.load("locales/.ftl.ron#en-US"));
    if asset_server.get_load_state(en) == Some(LoadState::Loaded) {
        let en = assets.get(en).unwrap();
        // From en-US bundle.
        assert!(matches!(en.content("hello-world"), Some(content) if content == "hello world"));
    }
    let ru = &*ru.get_or_insert_with(|| asset_server.load("locales/.ftl.ron#ru-RU"));
    if asset_server.get_load_state(ru) == Some(LoadState::Loaded) {
        let ru = assets.get(ru).unwrap();
        // From ru-RU bundle.
        assert!(matches!(ru.content("hello-world"), Some(content) if content == "привет мир"));
    }
}
