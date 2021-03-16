//! Resources module
//!
//! Any entity located directly in this module is
//! [`Resource`](bevy::ecs::Resource).

pub use self::{settings::Settings, snapshot::Snapshot};

pub(crate) use self::handles::Handles;

mod handles;
mod settings;
mod snapshot;
