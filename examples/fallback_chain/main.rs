use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::{prelude::*, resources::Locales};
use fluent_content::Content;
use unic_langid::langid;

pub fn main() {
    App::new()
        .insert_resource(
            Locales::new([langid!("en-US"), langid!("ru-RU"), langid!("ru-BY")])
                .with_default(langid!("en-US")),
        )
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "examples/fallback_chain/assets".to_string(),
                ..default()
            }),
            FluentPlugin,
        ))
        .add_systems(Update, localize)
        .run();
}

fn localize(
    locales: Res<Locales>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundleAsset>>,
    mut handles: Local<Option<Vec<Handle<BundleAsset>>>>,
    mut bundles: Local<Bundles>,
) {
    let handles = handles.get_or_insert_with(|| {
        locales
            .request(Some(langid!("ru-RU")))
            .iter()
            .map(|locale| asset_server.load(format!("locales/.ftl.ron#{locale}")))
            .collect()
    });
    if handles
        .iter()
        .all(|handle| asset_server.get_load_state(handle) == Some(LoadState::Loaded))
    {
        *bundles = handles
            .iter()
            .map(|handle| (handle.clone(), assets.get(handle).unwrap().clone()))
            .collect();
        // From ru-RU bundle, the first in fallback chain.
        assert!(matches!(bundles.content("hello"), Some(content) if content == "привет"));
        // From ru-BY bundle, the second in fallback chain.
        assert!(matches!(bundles.content("world"), Some(content) if content == "свету"));
        // From en-US bundle, the last in fallback chain, default locale.
        assert!(matches!(bundles.content("bevy"), Some(content) if content == "bevy"));
    }
}
