//! States module
//!
//! Any entity located directly in this module is [`State`](bevy::ecs::State).

/// Fluent state
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FluentState {
    LoadAssets,
    Snapshot,
    Done,
}
