//! Resources
//!
//! Any entity located directly in this module is
//! [`Resource`](bevy::ecs::system::Resource).

#[doc(inline)]
pub use self::{bundles::Bundles, locales::Locales};

mod bundles;
mod locales;
