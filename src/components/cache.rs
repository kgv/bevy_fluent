use crate::BundleAsset;
use bevy::{asset::HandleId, prelude::*};
use indexmap::IndexSet;
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// Cache
#[derive(Debug, Default)]
pub struct Cache(RwLock<HashMap<HandleId, IndexSet<Handle<BundleAsset>>>>);

impl Deref for Cache {
    type Target = RwLock<HashMap<HandleId, IndexSet<Handle<BundleAsset>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
