//! Assets
//!
//! Any entity located directly in this module is [`Asset`](bevy::asset::Asset).

pub use self::error::{Error, Result};
#[doc(inline)]
pub use self::{bundles::BundlesAsset, resource::ResourceAsset};

pub mod bundles;
pub mod error;
pub mod resource;
