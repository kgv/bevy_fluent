//! Bevy plugin for localization using Fluent.

#[cfg(not(feature = "implicit"))]
#[doc(inline)]
pub use self::assets::{locale::LocaleAssetsLoader, LocaleAssets};
#[doc(no_inline)]
pub use self::components::{Settings as SettingsComponent, Snapshot as SnapshotComponent};
#[doc(inline)]
pub use self::{
    assets::{fluent::FluentAssetLoader, FluentAsset},
    prelude::*,
};

pub(crate) use self::components::State as StateComponent;

/// `use bevy_fluent::prelude::*;` to import common assets, components and plugins
pub mod prelude {
    #[doc(inline)]
    pub use super::{
        components::Settings as FluentSettings, plugins::FluentPlugin, utils::bundle::Request,
    };
}

pub mod assets;
pub mod components;
pub mod plugins;
pub mod utils;

mod systems;
