//! Plugins
//!
//! Any entity located directly in this module is [`Plugin`](bevy::app::Plugin).

use crate::{
    assets::{bundle::BundleAssetLoader, resource::ResourceAssetLoader},
    BundleAsset, ResourceAsset,
};
use bevy::prelude::*;

/// Adds support for Fluent file loading to applications
#[derive(Default)]
pub struct FluentPlugin;

impl Plugin for FluentPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(ResourceAssetLoader)
            .init_asset::<ResourceAsset>()
            .register_asset_loader(BundleAssetLoader)
            .init_asset::<BundleAsset>();
    }
}
