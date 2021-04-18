//! Components module
//!
//! Any entity located directly in this module is
//! [`Component`](bevy::ecs::component::Component).

pub use self::{cache::Cache, queue::Queue};

mod cache;
mod queue;
