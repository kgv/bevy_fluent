//! Assets
//!
//! Any entity located directly in this module is [`Asset`](bevy::asset::Asset).

pub use self::error::{Error, Result};
#[doc(inline)]
pub use self::{bundle::BundleAsset, resource::ResourceAsset};

pub mod bundle;
pub mod error;
pub mod resource;
