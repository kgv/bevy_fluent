//! Systems module
//!
//! Any entity located directly in this module is
//! [`System`](bevy::ecs::system::System).

pub(crate) use self::serve::serve;

pub mod parameters;

mod serve;
