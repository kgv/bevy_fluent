#[cfg(not(feature = "implicit"))]
use crate::{assets::Bundle, BundleAssetLoader};
use crate::{
    assets::Resource,
    resources::{Handles, Settings},
    states::FluentState,
    systems::{check_assets, load_assets, snapshot},
    ResourceAssetLoader,
};
use bevy::prelude::*;

/// Adds support for Fluent file loading to Apps.
#[derive(Default)]
pub struct FluentPlugin;

impl Plugin for FluentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.world_mut()
            .get_resource_or_insert_with(Settings::default);
        #[cfg(not(feature = "implicit"))]
        app.init_asset_loader::<BundleAssetLoader>()
            .add_asset::<Bundle>();
        app.init_asset_loader::<ResourceAssetLoader>()
            .add_asset::<Resource>()
            .init_resource::<Handles>()
            .add_state(FluentState::LoadAssets)
            .add_system_set(
                SystemSet::on_enter(FluentState::LoadAssets).with_system(load_assets.system()),
            )
            .add_system_set(
                SystemSet::on_update(FluentState::LoadAssets).with_system(check_assets.system()),
            )
            .add_system_set(
                SystemSet::on_enter(FluentState::Snapshot).with_system(snapshot.system()),
            );
    }
}
