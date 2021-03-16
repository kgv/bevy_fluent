//! Systems module
//!
//! Any entity located directly in this module is
//! [`System`](bevy::ecs::system::System).

pub(crate) use self::{check_assets::check_assets, load_assets::load_assets, snapshot::snapshot};

mod check_assets;
mod load_assets;
mod snapshot;
