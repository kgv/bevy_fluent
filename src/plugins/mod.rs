use crate::{
    assets::{Bundle, Resource},
    BundleAssetLoader, ResourceAssetLoader,
};
use bevy::prelude::*;

/// Adds support for Fluent file loading to Apps.
#[derive(Default)]
pub struct Fluent;

impl Plugin for Fluent {
    fn build(&self, app: &mut AppBuilder) {
        app.init_asset_loader::<BundleAssetLoader>()
            .init_asset_loader::<ResourceAssetLoader>()
            .add_asset::<Bundle>()
            .add_asset::<Resource>();
    }
}
