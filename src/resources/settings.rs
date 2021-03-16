use unic_langid::{langid, LanguageIdentifier};

/// Settings.
#[derive(Clone, Debug)]
pub struct Settings {
    pub default_locale: Option<LanguageIdentifier>,
    /// Folder relative to the
    /// [`AssetServerSettings.asset_folder`](bevy::asset::AssetServerSettings.asset_folder)
    pub locale_folder: String,
    pub requested_locales: Vec<LanguageIdentifier>,
}

impl Settings {
    pub fn with_default_locale(self, default_locale: LanguageIdentifier) -> Self {
        Self {
            default_locale: Some(default_locale),
            ..self
        }
    }

    pub fn with_locale_folder<T: ToString>(self, locale_folder: T) -> Self {
        Self {
            locale_folder: locale_folder.to_string(),
            ..self
        }
    }

    pub fn with_requested_locales(self, requested_locales: Vec<LanguageIdentifier>) -> Self {
        Self {
            requested_locales,
            ..self
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_locale: Some(langid!("en-US")),
            locale_folder: "locales".to_string(),
            requested_locales: Vec::new(),
        }
    }
}
