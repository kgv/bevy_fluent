//! Localization asset

use std::{
    borrow::Borrow,
    fmt::{self, Debug, Formatter},
};

use bevy::{prelude::*, utils::tracing::instrument};
use fluent::{FluentArgs, FluentError};
use fluent_content::{Content, Request};
use indexmap::IndexMap;
use unic_langid::LanguageIdentifier;

use crate::{exts::fluent::BundleExt, BundleAsset};

/// Localization
///
/// Collection of [`BundleAsset`]s.
#[derive(Default, Resource)]
pub struct Localization(IndexMap<Handle<BundleAsset>, BundleAsset>);

impl Localization {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handles(&self) -> impl Iterator<Item = &Handle<BundleAsset>> {
        self.0.keys()
    }

    pub fn insert(&mut self, handle: &Handle<BundleAsset>, asset: &BundleAsset) {
        self.0.insert(handle.clone(), asset.clone());
    }

    pub fn locales(&self) -> impl Iterator<Item = &LanguageIdentifier> {
        self.0.values().map(|bundle| bundle.locale())
    }

    pub fn get_message(&self, id: &str, args: &FluentArgs, remove_lrm: bool) -> Option<String> {
        for bundle in self.0.values() {
            if let Some(msg) = bundle.get_message(id) {
                let mut errors: Vec<FluentError> = vec![];

                let value = bundle.format_pattern(&msg.value().unwrap(), Some(args), &mut errors);
                // Clean the output from any LRM symbols if asked for
                if remove_lrm {
                    let cleaned_value = value
                        .to_string()
                        .replace("\u{2068}", "")
                        .replace("\u{2069}", "");
                    return Some(cleaned_value);
                } else {
                    return Some(value.to_string());
                }
            }
        }
        None
    }
}

impl<'a, T, U> Content<'a, T, U> for Localization
where
    T: Copy + Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>>,
{
    #[instrument(fields(request = % request.into()), skip_all)]
    fn content(&self, request: T) -> Option<String> {
        self.0.values().find_map(|bundle| {
            let content = bundle.content(request);
            trace!(locale = %bundle.locale(), ?content);
            content
        })
    }
}

impl Debug for Localization {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Localization")
            .field(&self.locales().collect::<Vec<_>>())
            .finish()
    }
}
