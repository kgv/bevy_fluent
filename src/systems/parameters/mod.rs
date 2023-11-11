//! System parameters
//!
//! Any entity located directly in this module is
//! [`SystemParam`](bevy::ecs::system::SystemParam).

use crate::{exts::fluent::BundleExt, BundleAsset, Locale, Localization, ResourceAsset};
use bevy::{asset::LoadedFolder, ecs::system::SystemParam, prelude::*};
use std::{any::TypeId, collections::HashMap};

/// Localization builder
#[derive(SystemParam)]
pub struct LocalizationBuilder<'w> {
    loaded_folders: Res<'w, Assets<LoadedFolder>>,
    assets: Res<'w, Assets<BundleAsset>>,
    locale: Res<'w, Locale>,
}

impl LocalizationBuilder<'_> {
    pub fn build(&self, handle: &Handle<LoadedFolder>) -> Localization {
        let mut localization = Localization::new();
        if let Some(loaded_folder) = self.loaded_folders.get(handle) {
            let locale_entries: HashMap<_, _> = loaded_folder
                .handles
                .iter()
                .filter_map(|untyped_handle| {
                    if untyped_handle.type_id() != TypeId::of::<BundleAsset>() {
                        if untyped_handle.type_id() != TypeId::of::<ResourceAsset>() {
                            warn!(
                                r#""{:?}" locale folder contains not only `BundleAsset` or `ResourceAsset` "{:?}"."#,
                                handle.path(), untyped_handle.path()
                            );
                        }
                        return None;
                    }
                    // TODO
                    let typed_handle = untyped_handle.clone_weak().typed();
                    if let Some(asset) = self.assets.get(&typed_handle) {
                        Some((asset.locale(), Entry { handle: typed_handle, asset }))
                    } else {
                        error!(
                            "{:?} `BundleAsset` didn't receive.",
                            typed_handle.path(),
                        );
                        None
                    }
                })
                .collect();
            let locales = self.locale.fallback_chain(locale_entries.keys().cloned());
            for locale in locales {
                localization.insert(&locale_entries[locale].handle, locale_entries[locale].asset);
            }
        } else {
            error!("{:?} locale folder didn't load.", handle.path());
        }
        localization
    }
}

struct Entry<'a> {
    handle: Handle<BundleAsset>,
    asset: &'a BundleAsset,
}
