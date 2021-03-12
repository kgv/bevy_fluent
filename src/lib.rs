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

pub use self::{
    assets::{
        bundle::{Loader as BundleAssetLoader, Query},
        resource::Loader as ResourceAssetLoader,
        Bundle as BundleAsset, Resource as ResourceAsset,
    },
    error::{Error, Result},
    plugins::Fluent as FluentPlugin,
};

pub mod assets;
pub mod error;
pub mod plugins;
