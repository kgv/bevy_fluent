use unic_langid::{langid, LanguageIdentifier};

/// Settings
#[derive(Clone, Debug)]
pub struct Settings {
    /// The locale to use as the default in your application
    pub default_locale: Option<LanguageIdentifier>,
    /// The fallback locale chain you want to use in your application
    pub fallback_locale_chain: Vec<LanguageIdentifier>,
    /// Root folder for all locales
    ///
    /// Is a subfolder of
    /// [`AssetServerSettings.asset_folder`](bevy::asset::AssetServerSettings.asset_folder)
    pub locales_folder: String,
}

impl Settings {
    pub fn with_default_locale(self, default_locale: LanguageIdentifier) -> Self {
        Self {
            default_locale: Some(default_locale),
            ..self
        }
    }

    pub fn with_fallback_locale_chain(
        self,
        fallback_locale_chain: Vec<LanguageIdentifier>,
    ) -> Self {
        Self {
            fallback_locale_chain,
            ..self
        }
    }

    pub fn with_locales_folder<T: ToString>(self, locales_folder: T) -> Self {
        Self {
            locales_folder: locales_folder.to_string(),
            ..self
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_locale: Some(langid!("en-US")),
            fallback_locale_chain: Vec::new(),
            locales_folder: "locales".to_string(),
        }
    }
}
