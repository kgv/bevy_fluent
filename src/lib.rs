//! Bevy fluent
//!
//! Bevy plugin for localization using Fluent.

#[doc(inline)]
pub use self::{
    assets::{bundles::Default, BundlesAsset, ResourceAsset},
    plugins::FluentPlugin,
};

/// `use bevy_fluent::prelude::*;` to import common assets, components and plugins
pub mod prelude {
    #[doc(inline)]
    pub use super::{BundlesAsset, Default::Explicit as DefaultLocale, FluentPlugin};
}

pub mod assets;
pub mod plugins;
pub mod resources;
