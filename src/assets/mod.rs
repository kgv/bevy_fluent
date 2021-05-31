//! Assets module
//!
//! Any entity located directly in this module is [`Asset`](bevy::asset::Asset).

#[doc(inline)]
pub use self::{bundle::BundleAsset, localization::Localization, resource::ResourceAsset};

pub mod bundle;
pub mod localization;
pub mod resource;
