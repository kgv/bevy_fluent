//! Plugins
//!
//! Any entity located directly in this module is [`Plugin`](bevy::app::Plugin).

use crate::{
    assets::{bundle::BundleAssetLoader, resource::ResourceAssetLoader},
    systems::update_bundle_asset,
    BundleAsset, ResourceAsset,
};
use bevy::prelude::*;

/// Adds support for Fluent file loading to applications
#[derive(Default)]
pub struct FluentPlugin;

impl Plugin for FluentPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<ResourceAsset>()
            .init_asset_loader::<ResourceAssetLoader>()
            .add_asset::<BundleAsset>()
            .init_asset_loader::<BundleAssetLoader>() 
            .add_system(update_bundle_asset.in_base_set(CoreSet::PreUpdate));
    }
}
