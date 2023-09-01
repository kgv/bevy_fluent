use bevy::{
    asset::{LoadState, LoadedFolder},
    prelude::*,
};
use bevy_fluent::prelude::*;
use fluent_content::Content;
use unic_langid::langid;

pub fn main() {
    App::new()
        .insert_resource(Locale::new(langid!("ru-RU")).with_default(langid!("en-US")))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "examples/fallback_chain/assets".to_string(),
                ..default()
            }),
            FluentPlugin,
        ))
        .add_systems(Update, localized_hello_world)
        .run();
}

fn localized_hello_world(
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut handle: Local<Option<Handle<LoadedFolder>>>,
    mut localization: Local<Option<Localization>>,
) {
    let handle = &*handle.get_or_insert_with(|| asset_server.load_folder("locales"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let localization = localization.get_or_insert_with(|| localization_builder.build(handle));
        // From ru-RU bundle, the first in fallback chain.
        assert!(matches!(localization.content("hello"), Some(content) if content == "привет"));
        // From ru-BY bundle, the second in fallback chain.
        assert!(matches!(localization.content("world"), Some(content) if content == "свету"));
        // From en-US bundle, the last in fallback chain, default locale.
        assert!(
            matches!(localization.content("hello-world"), Some(content) if content == "hello world")
        );
    }
}
