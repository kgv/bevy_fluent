//! Bevy fluent
//!
//! Bevy plugin for localization using Fluent.

#[doc(inline)]
pub use self::{
    assets::{BundlesAsset, ResourceAsset},
    plugins::FluentPlugin,
};

/// `use bevy_fluent::prelude::*;` to import common assets, components and plugins
pub mod prelude {
    #[doc(inline)]
    pub use super::{BundlesAsset, FluentPlugin};
}

pub mod assets;
pub mod plugins;
pub mod resources;
