use bevy::prelude::{Font as BevyFont, *};
use std::ops::Deref;
use unic_langid::LanguageIdentifier;

/// Font
#[derive(Resource)]
pub struct Font(pub Handle<BevyFont>);

impl FromWorld for Font {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        Self(font)
    }
}

/// Locales
#[derive(Resource)]
pub struct Locales(pub Vec<LanguageIdentifier>);

impl Locales {
    pub fn index(&self, locale: &LanguageIdentifier) -> usize {
        self.iter()
            .position(|item| item == locale)
            .expect("index not found")
    }
}

impl Deref for Locales {
    type Target = Vec<LanguageIdentifier>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
