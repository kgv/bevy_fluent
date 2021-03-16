use crate::{resources::Snapshot, states::FluentState, utils::commands::CommandsExt};
use bevy::prelude::*;

pub(crate) fn snapshot(mut commands: Commands, mut state: ResMut<State<FluentState>>) {
    debug!("snapshot");
    commands.init_resource::<Snapshot>();
    state.set_next(FluentState::Done).unwrap();
}
