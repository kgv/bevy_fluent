use crate::GameState;
use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handles = asset_server
        .load_glob::<BundleAsset>("locales/**/menu.ftl.ron")
        .unwrap();
    commands.insert_resource(Handles(handles));
}

pub fn load(
    mut commands: Commands,
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<GameState>>,
    handles: Res<Handles>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(handles.0.iter().map(Handle::id)) {
        let localization = localization_builder.build(&handles.0);
        commands.remove_resource::<Handles>();
        commands.insert_resource(localization);
        game_state.set(GameState::Menu).unwrap();
    }
}

#[derive(Resource)]
pub struct Handles(Vec<Handle<BundleAsset>>);
