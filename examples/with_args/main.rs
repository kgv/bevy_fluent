use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::prelude::*;
use fluent::FluentArgs;
use unic_langid::langid;

pub fn main() {
    App::new()
        .insert_resource(Locale::new(langid!("de-DE")).with_default(langid!("en-US")))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            asset_folder: "examples/with_args/assets".to_string(),
            ..default()
        }))
        .add_plugins(FluentPlugin)
        .add_systems(Update, localized_with_args)
        .run();
}

fn localized_with_args(
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

        let mut args = FluentArgs::new();
        args.set("name", "world");
        let message = localization.get_message("hello-name", &args, true).unwrap();
        assert!(matches!(message, v if v == "Hello, world!"));

        args.set("count", 0);
        let message = localization
            .get_message("apple-count", &args, true)
            .unwrap();
        assert!(matches!(message, v if v == "Ich habe keine Äpfel."));

        let mut args = FluentArgs::new();
        args.set("count", 1);
        let message = localization
            .get_message("apple-count", &args, true)
            .unwrap();
        assert!(matches!(message, v if v == "Ich habe einen Apfel."));

        let mut args = FluentArgs::new();
        args.set("count", 2);
        let message = localization
            .get_message("apple-count", &args, true)
            .unwrap();
        assert!(matches!(message, v if v == "Ich habe 2 Äpfel."));
    }
}
