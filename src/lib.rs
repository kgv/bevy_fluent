//! Bevy fluent
//!
//! Bevy plugin for localization using Fluent.

#[doc(inline)]
pub use self::{
    assets::{BundleAsset, ResourceAsset},
    plugins::FluentPlugin,
    resources::{Bundles, Locales},
};

/// `use bevy_fluent::prelude::*;` to import common assets, components and plugins
pub mod prelude {
    #[doc(inline)]
    pub use super::{BundleAsset, Bundles, FluentPlugin, Locales};
}

pub mod assets;
pub mod plugins;
pub mod resources;
