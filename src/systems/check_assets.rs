use crate::{components::Handles, StateComponent};
use bevy::{
    app::{AppExit, Events},
    asset::LoadState,
    prelude::*,
};

pub(crate) fn check_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: ResMut<Events<AppExit>>,
    handles: Res<Handles>,
    mut state: ResMut<State<StateComponent>>,
) {
    debug!("check assets");
    match asset_server.get_group_load_state(handles.iter().map(|handle| handle.id)) {
        LoadState::Failed => events.send(AppExit),
        LoadState::Loaded => {
            debug!("assets are loaded");
            commands.remove_resource::<Handles>();
            state.set_next(StateComponent::Snapshot).unwrap();
        }
        _ => {}
    }
}
