//! System parameters module
//!
//! Any entity located directly in this module is
//! [`SystemParam`](bevy::ecs::system::SystemParam).

pub use self::server::FluentServer;

mod server;
