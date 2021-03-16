use bevy::prelude::HandleUntyped;
use std::ops::Deref;

#[derive(Default)]
pub(crate) struct Handles(pub(crate) Vec<HandleUntyped>);

impl Deref for Handles {
    type Target = Vec<HandleUntyped>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
