//! Systems
//!
//! Any entity located directly in this module is
//! [`System`](bevy::ecs::system::System).

use crate::{BundleAsset, ResourceAsset};
use bevy::prelude::{warn, AssetEvent, Assets, EventReader, Handle, Res, ResMut};
use fluent::bundle::FluentBundle;
use std::sync::Arc;

/// Re-loads bundle assets when the resources they depend on changes.
pub(crate) fn update_bundle_asset(
    mut resource_updates: EventReader<AssetEvent<ResourceAsset>>,
    mut bundle_assets: ResMut<Assets<BundleAsset>>,
    resource_assets: Res<Assets<ResourceAsset>>,
) {
    for event in resource_updates.iter() {
        let mut bundles_to_update = Vec::new();

        // If a resource asset is modified
        if let AssetEvent::Modified { handle } = event {
            // Look for all the bundles that that resource was used in
            for (bundle_id, bundle_asset) in bundle_assets.iter() {
                for resource_handle in &bundle_asset.resource_handles {
                    if handle.id() == resource_handle.id() {
                        bundles_to_update.push(Handle::weak(bundle_id));
                    }
                }
            }

            // Update all bundles that included the resource
            for handle in bundles_to_update {
                // Get a mutable reference to the old bundle
                let bundle = bundle_assets.get_mut(&handle).unwrap();

                // Create a new bundle to replace it
                let mut new_bundle = FluentBundle::new_concurrent(bundle.locales.clone());

                // Add all resources from the old bundle to the new bundle
                for resource_handle in &bundle.resource_handles {
                    let resource = resource_assets.get(resource_handle).unwrap();
                    if let Err(errors) = new_bundle.add_resource(resource.0.clone()) {
                        for e in errors {
                            // Skip overriding errors, because we specifically want to override any
                            // updated messages.
                            if !matches!(e, fluent::FluentError::Overriding { .. }) {
                                warn!("Error loading fluent resource: {}", e);
                            }
                        }
                    }
                }

                // Update the old bundle
                bundle.bundle = Arc::new(new_bundle);
            }
        }
    }
}

pub mod parameters;
