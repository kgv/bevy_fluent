use bevy::prelude::*;
use std::ops::Deref;

/// Collection of any asset handles loaded from `assets/locales` directory
#[derive(Default)]
pub(crate) struct Handles(pub(crate) Vec<HandleUntyped>);

impl Deref for Handles {
    type Target = Vec<HandleUntyped>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
