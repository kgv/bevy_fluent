use bevy::prelude::*;
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

/// Locales fallback chain
#[derive(Clone, Debug, Default, Deserialize, Resource, Serialize)]
pub struct FallbackChain<A = LanguageIdentifier> {
    pub available: Vec<A>,
    pub default: Option<A>,
}

impl<A> FallbackChain<A> {
    /// Creates a new fallback chain. Sets the available locales.
    pub fn new(available: impl IntoIterator<Item = A>) -> Self {
        Self {
            available: Vec::from_iter(available),
            default: None,
        }
    }

    /// Set the default locale.
    pub fn with_default(self, default: A) -> Self {
        Self {
            default: Some(default),
            ..self
        }
    }
}

impl<A: AsRef<LanguageIdentifier> + PartialEq> FallbackChain<A> {
    /// Receives requested locales. Returns supported locales fallback chain.
    pub fn request<'a, R: 'a + AsRef<LanguageIdentifier>>(
        &'a self,
        requested: impl IntoIterator<Item = R>,
    ) -> Vec<&'a A> {
        let requested = &Vec::from_iter(requested);
        let available = &self.available;
        let default = self.default.as_ref();
        let supported = negotiate_languages(
            requested,
            available,
            default,
            NegotiationStrategy::Filtering,
        );
        supported
    }
}
