//! Bevy fluent
//!
//! Bevy plugin for localization using Fluent.

#[doc(inline)]
pub use self::{
    assets::{
        bundle::BundleAssetLoader, resource::ResourceAssetLoader, BundleAsset, Localization,
        ResourceAsset,
    },
    plugins::FluentPlugin,
    systems::parameters::FluentServer,
};

/// `use bevy_fluent::prelude::*;` to import common assets, components and plugins
pub mod prelude {
    #[doc(inline)]
    pub use super::{exts::BundleExt, FluentPlugin, FluentServer, Localization};
}

pub mod assets;
pub mod components;
pub mod exts;
pub mod plugins;
pub mod systems;
