//! Resources
//!
//! Any entity located directly in this module is
//! [`Resource`](bevy::ecs::system::Resource).

use bevy::prelude::*;
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use std::slice::from_ref;
use unic_langid::LanguageIdentifier;

/// Locale
#[derive(Clone, Debug, Default)]
pub struct Locale {
    pub requested: LanguageIdentifier,
    pub default: Option<LanguageIdentifier>,
}

impl Locale {
    pub fn new(locale: LanguageIdentifier) -> Self {
        Self {
            requested: locale,
            default: None,
        }
    }

    pub fn with_default(mut self, locale: LanguageIdentifier) -> Self {
        self.default = Some(locale);
        self
    }

    pub fn fallback_chain<'a, I>(&'a self, locales: I) -> Vec<&'a LanguageIdentifier>
    where
        I: Iterator<Item = &'a LanguageIdentifier>,
    {
        let available = &locales.collect::<Vec<_>>();
        let default = self.default.as_ref();
        let requested = from_ref(&self.requested);
        let supported = negotiate_languages(
            requested,
            available,
            default.as_ref(),
            NegotiationStrategy::Filtering,
        );
        debug!(
            requested = ?requested.iter().map(|locale| format!("{locale}")).collect::<Vec<_>>(),
            available = ?available.iter().map(|locale| format!("{locale}")).collect::<Vec<_>>(),
            default = ?default.map(|locale| format!("{locale}")),
            supported = ?supported.iter().map(|locale| format!("{locale}")).collect::<Vec<_>>(),
        );
        supported.into_iter().copied().collect()
    }
}
