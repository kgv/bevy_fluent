use crate::{utils::commands::CommandsExt, FluentState, Snapshot};
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};

#[instrument(fields(state = ?*state), skip(commands))]
pub(crate) fn take_snapshot(mut commands: Commands, mut state: ResMut<State<FluentState>>) {
    trace!("call");
    commands.init_resource::<Snapshot>();
    state.set_next(FluentState::Done).unwrap();
}
