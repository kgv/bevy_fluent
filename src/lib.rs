//! Bevy plugin for localization using Fluent.
//!
//! # Definitions
//!
//! Fluent:
//!
//! The basic unit of translation in Fluent is called a ***message***. Each
//! message has an ***identifier***. *Messages* (and terms, variants,
//! attributes) store their values as ***patterns***.
//!
//! Local:
//!
//! Formated *pattern* are called ***content***. ***Query*** provides access to
//! *content* according to the given components.

#![feature(iter_intersperse)]

#[cfg(not(feature = "implicit"))]
#[doc(no_inline)]
pub use self::assets::{bundle::Loader as BundleAssetLoader, Bundle as BundleAsset};
#[doc(no_inline)]
pub use self::{
    assets::{resource::Loader as ResourceAssetLoader, Resource as ResourceAsset},
    resources::{Settings as SettingsResource, Snapshot as SnapshotResource},
};

#[doc(inline)]
pub use self::{
    prelude::*,
    resources::{Settings, Snapshot},
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
