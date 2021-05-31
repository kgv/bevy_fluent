use crate::BundleAsset;
use bevy::{asset::HandleId, prelude::*};
use indexmap::IndexSet;
use parking_lot::RwLock;
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

/// Queue
#[derive(Debug, Default)]
pub struct Queue(RwLock<VecDeque<(HandleId, IndexSet<Handle<BundleAsset>>)>>);

impl Deref for Queue {
    type Target = RwLock<VecDeque<(HandleId, IndexSet<Handle<BundleAsset>>)>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Queue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
