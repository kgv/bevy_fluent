use bevy::{
    asset::{AssetServerSettings, LoadState},
    prelude::*,
};
use bevy_fluent::prelude::*;
use unic_langid::langid;

pub fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            asset_folder: "examples/minimal/assets".to_string(),
            ..default()
        })
        .insert_resource(Locale::new(langid!("en-US")))
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .add_system(localized_hello_world)
        .run();
}

fn localized_hello_world(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundleAsset>>,
    mut handle: Local<Option<Handle<BundleAsset>>>,
) {
    let handle = &*handle.get_or_insert_with(|| asset_server.load("locales/en-US/main.ftl.yml"));
    if let LoadState::Loaded = asset_server.get_load_state(handle) {
        let bundle = assets.get(handle).unwrap();
        assert!(matches!(bundle.content("hello-world"), Some(content) if content == "hello world"));
    }
}
