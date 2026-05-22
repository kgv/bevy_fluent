//! Loads a Fluent bundle from `embedded://` assets — headless, no window.
//!
//! Demonstrates that the bundle loader preserves the `embedded` asset source
//! when resolving the YAML's sibling `.ftl` dependency.

use bevy::{
    app::AppExit,
    asset::{embedded_asset, AssetPlugin, LoadState},
    log::LogPlugin,
    prelude::*,
};
use bevy_fluent::prelude::*;
use fluent_content::Content;
use unic_langid::langid;

pub fn main() {
    App::new()
        .insert_resource(Locale::new(langid!("en-US")))
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            LogPlugin::default(),
            EmbeddedLocalesPlugin,
            FluentPlugin,
        ))
        .add_systems(Update, localized_hello_world)
        .run();
}

/// See the comments below — `embedded_asset!` from inside a cargo example
/// has two non-obvious quirks (src prefix and crate name).
struct EmbeddedLocalesPlugin;

impl Plugin for EmbeddedLocalesPlugin {
    fn build(&self, app: &mut App) {
        // 3-arg form: examples aren't under `src/`, so override the prefix
        // that gets stripped from the calling file's path.
        embedded_asset!(
            app,
            "examples/embedded",
            "assets/locales/en-US/main.ftl.yml"
        );
        embedded_asset!(
            app,
            "examples/embedded",
            "assets/locales/en-US/hello_world.ftl"
        );
    }
}

fn localized_hello_world(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundleAsset>>,
    mut handle: Local<Option<Handle<BundleAsset>>>,
    mut exit: MessageWriter<AppExit>,
) {
    // `module_path!()` resolves to the example's own crate name (`embedded`),
    // not `bevy_fluent` — that's why the URL's host segment is `embedded`.
    let handle = &*handle.get_or_insert_with(|| {
        asset_server.load("embedded://embedded/assets/locales/en-US/main.ftl.yml")
    });
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let bundle = assets.get(handle).unwrap();
        let content = bundle.content("hello-world");
        info!(?content, "loaded embedded bundle");
        assert!(matches!(&content, Some(s) if s == "hello world (embedded)"));
        exit.write(AppExit::Success);
    }
}
