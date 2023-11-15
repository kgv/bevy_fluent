use crate::{
    resources::{Loaded, Loading, Locales},
    GameState,
};
use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::{
    assets::bundles::{Locale, Settings},
    BundlesAsset,
};

pub fn setup(
    mut commands: Commands,
    // mut assets: ResMut<Assets<BundlesAsset>>,
    asset_server: Res<AssetServer>,
    // loaded: Option<Res<Loaded>>,
    locales: Res<Locales>,
) {
    // if let Some(handle) = handle {
    //     error!(remove = ?handle.0.path());
    //     assets.remove(&handle.0);
    //     commands.remove_resource::<Loaded>();
    // }
    let locale = locales[0].clone();
    error!(%locale);
    let handle =
        asset_server.load_with_settings("locales/.ftl.ron", move |settings: &mut Settings| {
            settings.locales.push(Locale {
                requested: vec![locale.clone()],
                ..default()
            });
        });
    // if loaded.is_some() {
    //     asset_server.asset_server.reload("locales/.ftl.ron")
    // }
    error!(new = ?handle.path());
    commands.insert_resource(Loading(handle));
}

pub fn update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    handle: Res<Loading>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if asset_server.get_load_state(&handle.0) == Some(LoadState::Loaded) {
        commands.remove_resource::<Loading>();
        commands.insert_resource(Loaded(handle.0.clone()));
        next_state.set(GameState::Menu);
    }
}
