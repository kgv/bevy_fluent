//! Plugins module
//!
//! Any entity located directly in this module is [`Plugin`](bevy::app::Plugin).

use crate::{
    components::{Cache, Queue},
    systems::serve,
    BundleAsset, BundleAssetLoader, Localization, ResourceAsset, ResourceAssetLoader,
};
use bevy::{ecs::system::IntoExclusiveSystem, prelude::*};

/// Adds support for Fluent file loading to applications
#[derive(Default)]
pub struct FluentPlugin;

impl Plugin for FluentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ResourceAsset>()
            .init_asset_loader::<ResourceAssetLoader>()
            .add_asset::<BundleAsset>()
            .init_asset_loader::<BundleAssetLoader>()
            .add_asset::<Localization>()
            .init_resource::<Cache>()
            .init_resource::<Queue>()
            .add_system_to_stage(CoreStage::PostUpdate, serve.exclusive_system());
    }
}
