use crate::{resources::Handles, GameState};
use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, locales: Res<Locales>) {
    let handles = Handles(
        locales
            .request(Some(&locales.available[0]))
            .iter()
            .map(|locale| asset_server.load(format!("locales/.ftl.ron#{locale}")))
            .collect(),
    );
    commands.insert_resource(handles);
}

pub fn update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundleAsset>>,
    handles: Res<Handles>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if handles
        .iter()
        .all(|handle| asset_server.get_load_state(handle) == Some(LoadState::Loaded))
    {
        let bundles = Bundles(
            handles
                .iter()
                .map(|handle| (handle.clone(), assets.get(handle).unwrap().clone()))
                .collect(),
        );
        commands.remove_resource::<Handles>();
        commands.insert_resource(bundles);
        next_state.set(GameState::Menu);
    }
}
