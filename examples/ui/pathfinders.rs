use crate::{
    components::Locales,
    locales::{de, en, ru},
};
use std::{iter::once, path::Path};
use unic_langid::LanguageIdentifier;

/// Pathfinder
pub trait Pathfinder {
    fn path(locale: Option<&LanguageIdentifier>) -> &'static Path;

    fn paths(locales: &Locales) -> Vec<&'static Path> {
        locales
            .fallback_chain()
            .into_iter()
            .map(Some)
            .chain(once(None))
            .map(Self::path)
            .collect()
    }
}

/// Menu pathfinder
pub struct Menu;

impl Pathfinder for Menu {
    fn path(locale: Option<&LanguageIdentifier>) -> &'static Path {
        match locale {
            Some(&de::DE) => Path::new("locales/de-DE/menu.ron"),
            Some(&en::US) => Path::new("locales/en-US/menu.ron"),
            Some(&ru::BY) => Path::new("locales/ru/ru-BY/menu.ron"),
            Some(&ru::RU) => Path::new("locales/ru/ru-RU/menu.ron"),
            None => Path::new("locales/interlocale/menu.ron"),
            _ => unimplemented!(),
        }
    }
}
