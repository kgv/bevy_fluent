//! Assets module
//!
//! Any entity located directly in this module is [`Asset`](bevy::asset::Asset).

#[doc(inline)]
pub use self::fluent::FluentAsset;
#[cfg(not(feature = "implicit"))]
#[doc(inline)]
pub use self::locale::LocaleAssets;

pub mod fluent;
#[cfg(not(feature = "implicit"))]
pub mod locale;
