//! Plugins module
//!
//! Any entity located directly in this module is [`Plugin`](bevy::app::Plugin).

use crate::{
    components::{Settings, State},
    systems::{check_assets, init_resources, load_assets, take_snapshot},
    FluentAsset, FluentAssetLoader,
};
#[cfg(not(feature = "implicit"))]
use crate::{LocaleAssets, LocaleAssetsLoader};
use bevy::prelude::*;

/// Adds support for Fluent file loading to Apps
#[derive(Default)]
pub struct FluentPlugin;

impl Plugin for FluentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.world_mut()
            .get_resource_or_insert_with(Settings::default);
        #[cfg(not(feature = "implicit"))]
        app.init_asset_loader::<LocaleAssetsLoader>()
            .add_asset::<LocaleAssets>();
        app.init_asset_loader::<FluentAssetLoader>()
            .add_asset::<FluentAsset>()
            .add_state_to_stage(CoreStage::PreUpdate, State::InitResources)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_enter(State::InitResources).with_system(init_resources.system()),
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_enter(State::LoadAssets).with_system(load_assets.system()),
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_update(State::LoadAssets).with_system(check_assets.system()),
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_enter(State::TakeSnapshot).with_system(take_snapshot.system()),
            );
    }
}
