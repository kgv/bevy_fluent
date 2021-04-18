use unic_langid::{langid, LanguageIdentifier};

pub mod de {
    use super::*;

    pub const DE: LanguageIdentifier = langid!("de-DE");
}

pub mod en {
    use super::*;

    pub const US: LanguageIdentifier = langid!("en-US");
}

pub mod ru {
    use super::*;

    pub const BY: LanguageIdentifier = langid!("ru-BY");
    pub const RU: LanguageIdentifier = langid!("ru-RU");
}
