use std::ops::Deref;
use unic_langid::LanguageIdentifier;

/// Locales
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
