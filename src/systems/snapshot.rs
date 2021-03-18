use crate::{components::Snapshot, utils::commands::CommandsExt, StateComponent};
use bevy::prelude::*;

pub(crate) fn snapshot(mut commands: Commands, mut state: ResMut<State<StateComponent>>) {
    debug!("snapshot");
    commands.init_resource::<Snapshot>();
    state.set_next(StateComponent::Done).unwrap();
}
