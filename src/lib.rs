//! Bevy plugin for localization using Fluent.

#![feature(iter_intersperse)]

#[cfg(not(feature = "implicit"))]
#[doc(inline)]
pub use self::assets::{locale::LocaleAssetsLoader, LocaleAssets};
#[doc(no_inline)]
pub use self::resources::{Settings as SettingsResource, Snapshot as SnapshotResource};
#[doc(inline)]
pub use self::{
    assets::{fluent::FluentAssetLoader, FluentAsset},
    prelude::*,
};

/// `use bevy_fluent::prelude::*;` to import common assets, resources and plugins
pub mod prelude {
    #[doc(inline)]
    pub use super::{
        plugins::FluentPlugin, resources::Settings as FluentSettings,
        utils::bundle::Query as FluentQuery,
    };
}

pub mod assets;
pub mod plugins;
pub mod resources;
pub mod utils;

mod states;
mod systems;
