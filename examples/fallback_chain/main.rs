use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::prelude::*;
use fluent_content::Content;
use unic_langid::langid;

pub fn main() {
    App::new()
        .insert_resource(Locale::new(langid!("ru-RU")).with_default(langid!("en-US")))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            asset_folder: "examples/fallback_chain/assets".to_string(),
            ..default()
        }))
        .add_plugin(FluentPlugin)
        .add_system(localized_hello_world)
        .run();
}

fn localized_hello_world(
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut localization: Local<Option<Localization>>,
    mut handles: Local<Option<Vec<Handle<BundleAsset>>>>,
) {
    let handles =
        handles.get_or_insert_with(|| asset_server.load_glob("locales/**/main.ftl.ron").unwrap());
    let load_state = asset_server.get_group_load_state(handles.iter().map(Handle::id));
    if let LoadState::Loaded = load_state {
        let localization =
            localization.get_or_insert_with(|| localization_builder.build(&*handles));
        // From ru-RU bundle, the first in fallback chain.
        assert!(matches!(localization.content("hello"), Some(v) if v == "привет"));
        // From ru-BY bundle, the second in fallback chain.
        assert!(matches!(localization.content("world"), Some(v) if v == "свету"));
        // From en-US bundle, the last in fallback chain, default locale.
        assert!(matches!(localization.content("hello-world"), Some(v) if v == "hello world"));
    }
}
