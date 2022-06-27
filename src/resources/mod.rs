//! Resources
//!
//! Any entity located directly in this module is
//! [`Resource`](bevy::ecs::system::Resource).

#[doc(inline)]
pub use self::{locale::Locale, localization::Localization};

mod locale;
mod localization;
