//! Components module
//!
//! Any entity located directly in this module is
//! [`Component`](bevy::ecs::component::Component).

pub use self::{settings::Settings, snapshot::Snapshot, state::State};

pub(crate) use self::handles::Handles;

mod handles;
mod settings;
mod snapshot;
mod state;
