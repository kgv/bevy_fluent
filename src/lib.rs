//! Bevy fluent
//!
//! Bevy plugin for localization using Fluent.

#[cfg(not(feature = "implicit"))]
#[doc(inline)]
pub use self::assets::{locale::LocaleAssetsLoader, LocaleAssets};
#[doc(inline)]
pub use self::{
    assets::{fluent::FluentAssetLoader, FluentAsset},
    components::{FluentSettings, FluentState, Snapshot},
    plugins::FluentPlugin,
};

/// `use bevy_fluent::prelude::*;` to import common assets, components and plugins
pub mod prelude {
    #[doc(inline)]
    pub use super::{utils::bundle::Request, FluentPlugin, FluentSettings, FluentState};
}

pub mod assets;
pub mod components;
pub mod plugins;
pub mod utils;

mod systems;
