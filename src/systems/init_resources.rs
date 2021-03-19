use crate::{components::Handles, utils::commands::CommandsExt, StateComponent};
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};

#[instrument(fields(state = ?*state), skip(commands))]
pub(crate) fn init_resources(mut commands: Commands, mut state: ResMut<State<StateComponent>>) {
    trace!("call");
    commands.init_resource::<Handles>();
    state.set_next(StateComponent::LoadAssets).unwrap();
}
