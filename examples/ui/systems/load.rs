use crate::{
    components::Locales,
    pathfinders::{Menu as MenuPathfinder, Pathfinder},
    GameState,
};
use bevy::prelude::*;
use bevy_fluent::prelude::*;

pub fn setup(mut commands: Commands, fluent_server: FluentServer, locales: Res<Locales>) {
    let paths = MenuPathfinder::paths(&locales);
    let handle = fluent_server.load(paths);
    commands.insert_resource(handle);
}

pub fn loading(
    assets: Res<Assets<Localization>>,
    mut game_state: ResMut<State<GameState>>,
    handle: ResMut<Handle<Localization>>,
) {
    if assets.get(handle.id).is_some() {
        game_state.set(GameState::Menu).unwrap();
    }
}
