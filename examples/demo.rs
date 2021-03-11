use bevy::prelude::*;
use bevy_fluent::{BundleAsset, FluentPlugin, Query};

pub fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .init_resource::<State>()
        .add_startup_system(setup.system())
        .add_system(print.system())
        .run();
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handles.interlocale = asset_server.load("locales/bundle.ron");
    state.handles.en = asset_server.load("locales/en-US/bundle.ron");
    state.handles.ru = asset_server.load("locales/ru-RU/bundle.ron");
}

fn print(state: ResMut<State>, bundle_assets: ResMut<Assets<BundleAsset>>) {
    let query = Query::builder().id("bevy").build();
    if let Some(bundle) = bundle_assets.get(&state.handles.interlocale) {
        if let Some(content) = bundle.content(&query) {
            println!("Bevy: {}.", content);
        }
    }
    let query = Query::builder().id("one").build();
    if let Some(bundle) = bundle_assets.get(&state.handles.en) {
        if let Some(content) = bundle.content(&query) {
            println!("One (en): {}.", content);
        }
    }
    if let Some(bundle) = bundle_assets.get(&state.handles.ru) {
        if let Some(content) = bundle.content(&query) {
            println!("One (ru): {}.", content);
        }
    }
}

#[derive(Default)]
struct State {
    handles: Handles,
}

#[derive(Default)]
struct Handles {
    interlocale: Handle<BundleAsset>,
    en: Handle<BundleAsset>,
    ru: Handle<BundleAsset>,
}
