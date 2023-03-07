use crate::resources::Locales;
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_fluent::*;

/// Swipe to one of the next or previous locale
#[derive(SystemParam)]
pub struct Swiper<'w> {
    locale: ResMut<'w, Locale>,
    locales: Res<'w, Locales>,
}

impl Swiper<'_> {
    pub fn next(&mut self) {
        let mut index = self.locales.index(&self.locale.requested);
        index = index.saturating_add(1).min(self.locales.len() - 1);
        self.locale.requested = self.locales[index].clone();
    }

    pub fn previous(&mut self) {
        let mut index = self.locales.index(&self.locale.requested);
        index = index.saturating_sub(1);
        self.locale.requested = self.locales[index].clone();
    }
}
