//! Systems module
//!
//! Any entity located directly in this module is
//! [`System`](bevy::ecs::system::System).

pub(crate) use self::{
    check_assets::check_assets, init_resources::init_resources, load_assets::load_assets,
    take_snapshot::take_snapshot,
};

mod check_assets;
mod init_resources;
mod load_assets;
mod take_snapshot;
