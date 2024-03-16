use bevy::asset::LoadDirectError;
use ron::error::SpannedError;
use std::{io, path::PathBuf};
use thiserror::Error;
use unic_langid::LanguageIdentifier;

/// Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    LoadDirect(#[from] LoadDirectError),
    #[error(transparent)]
    Ron(#[from] SpannedError),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error("Locale not found {{ locale: {locale}, path: {path} }}.")]
    LocaleNotFound {
        locale: LanguageIdentifier,
        path: PathBuf,
    },
}
