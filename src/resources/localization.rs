//! Localization asset

use crate::{exts::fluent::BundleExt, BundleAsset};
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};
use fluent::FluentArgs;
use fluent_content::{Content, Request};
use indexmap::IndexMap;
use std::{
    borrow::Borrow,
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};
use unic_langid::LanguageIdentifier;

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
}

impl<'a, T, U> Content<'a, T, U> for Localization
where
    T: Copy + Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>> + Debug,
{
    #[instrument(fields(request = %request.into()), skip_all)]
    fn content(&self, request: T) -> Option<String> {
        self.0.values().find_map(|bundle| {
            let request = request.into();
            let request = request.borrow();
            let message = bundle.get_message(request.id)?;

            let pattern = match request.attr {
                Some(key) if !key.is_empty() => message.get_attribute(key)?.value(),
                _ => message.value()?,
            };

            let mut errors = Vec::new();
            let content = bundle
                .format_pattern(
                    pattern,
                    request.args.as_ref().map(Borrow::borrow),
                    &mut errors,
                )
                .to_string();

            for error in &errors {
                error!(%error);
            }

            Some(content.replace("\u{2068}", "").replace("\u{2069}", "")) // Dirty hack to strip any "First Strong Isolate" unicode values
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

impl Deref for Localization {
    type Target = IndexMap<Handle<BundleAsset>, BundleAsset>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Localization {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
