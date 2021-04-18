use bevy::prelude::*;
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use std::slice::from_ref;
use unic_langid::LanguageIdentifier;

/// Color materials
pub struct ColorMaterials {
    pub gray25: Handle<ColorMaterial>,
    pub gray50: Handle<ColorMaterial>,
    pub gray75: Handle<ColorMaterial>,
    pub none: Handle<ColorMaterial>,
}

impl FromWorld for ColorMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut color_material = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let gray25 = color_material.add(Color::rgb(0.25, 0.25, 0.25).into());
        let gray50 = color_material.add(Color::rgb(0.50, 0.50, 0.50).into());
        let gray75 = color_material.add(Color::rgb(0.75, 0.75, 0.75).into());
        let none = color_material.add(Color::NONE.into());
        Self {
            gray25,
            gray50,
            gray75,
            none,
        }
    }
}

/// Default font
pub struct DefaultFont(pub Handle<Font>);

impl FromWorld for DefaultFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        Self(font)
    }
}

/// Locales
///
/// Represents all available locales, the currently locale and the default
/// locale. By default the current locale is the first available locale. You can
/// select the current locale with [`shift`](Locales::shift). You can set the
/// default locale when the [`Locales`] is created with
/// [`with_default`](Locales::with_default).
#[derive(Clone, Debug)]
pub struct Locales {
    available: Vec<LanguageIdentifier>,
    current: usize,
    default: Option<usize>,
}

impl Locales {
    pub fn new(locale: LanguageIdentifier) -> Self {
        Self {
            available: vec![locale],
            current: 0,
            default: None,
        }
    }

    pub fn with(mut self, locale: LanguageIdentifier) -> Self {
        self.available.push(locale);
        self
    }

    pub fn with_default(mut self, locale: LanguageIdentifier) -> Self {
        self.default = Some(self.available.len());
        self.available.push(locale);
        self
    }

    pub fn available(&self) -> &[LanguageIdentifier] {
        &self.available
    }

    pub fn current(&self) -> &LanguageIdentifier {
        &self.available[self.current]
    }

    pub fn default(&self) -> Option<&LanguageIdentifier> {
        Some(&self.available[self.default?])
    }

    pub fn fallback_chain(&self) -> Vec<&LanguageIdentifier> {
        let available = self.available();
        let default = self.default();
        let requested = from_ref(self.current());
        negotiate_languages(
            requested,
            available,
            default,
            NegotiationStrategy::Filtering,
        )
    }

    /// Shift to one of the next or previous locale
    pub fn shift(&mut self, count: isize) {
        if count.is_positive() {
            let count = count as _;
            self.current = self
                .current
                .saturating_add(count)
                .min(self.available.len() - 1);
        } else if count.is_negative() {
            let count = count.abs() as _;
            self.current = self.current.saturating_sub(count);
        }
    }
}

pub mod tags;
