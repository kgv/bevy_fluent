use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_fluent::prelude::*;

pub fn main() {
    App::build()
        .insert_resource(AssetServerSettings {
            asset_folder: "examples/minimal/assets".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .add_system(localized_hello_world.system())
        .run();
}

fn localized_hello_world(
    fluent_server: FluentServer,
    assets: Res<Assets<Localization>>,
    mut handle: Local<Option<Handle<Localization>>>,
) {
    let localization_handle =
        handle.get_or_insert_with(|| fluent_server.load(vec!["locales/en-US/locale.ron"]));
    if let Some(localization) = assets.get(localization_handle.id) {
        let hello_world = localization.content("hello-world").unwrap();
        println!("{}", hello_world);
    }
}
