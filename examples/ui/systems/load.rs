use crate::GameState;
use bevy::{
    asset::{LoadState, LoadedFolder},
    prelude::*,
};
use bevy_fluent::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load_folder("locales");
    commands.insert_resource(LocaleFolder(handle));
}

pub fn update(
    mut commands: Commands,
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    locale_folder: Res<LocaleFolder>,
) {
    if let Some(LoadState::Loaded) = asset_server.get_load_state(&locale_folder.0) {
        let localization = localization_builder.build(&locale_folder.0);
        commands.remove_resource::<LocaleFolder>();
        commands.insert_resource(localization);
        next_state.set(GameState::Menu);
    }
}

#[derive(Resource)]
pub struct LocaleFolder(Handle<LoadedFolder>);
