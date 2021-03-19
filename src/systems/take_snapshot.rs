use crate::{components::Snapshot, utils::commands::CommandsExt, StateComponent};
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};

#[instrument(fields(state = ?*state), skip(commands))]
pub(crate) fn take_snapshot(mut commands: Commands, mut state: ResMut<State<StateComponent>>) {
    trace!("call");
    commands.init_resource::<Snapshot>();
    state.set_next(StateComponent::Done).unwrap();
}
