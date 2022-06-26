//! Localization asset

use crate::{
    exts::fluent::{content::Request, BundleExt, Content},
    BundleAsset,
};
use bevy::{
    asset::HandleId,
    prelude::*,
    reflect::TypeUuid,
    utils::tracing::{self, instrument},
};
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use indexmap::IndexMap;
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::{
    borrow::Borrow,
    fmt::{self, Debug, Formatter},
    sync::Arc,
};
use unic_langid::LanguageIdentifier;

/// Collection of [`FluentBundle`](fluent::bundle::FluentBundle)s
#[derive(TypeUuid)]
#[uuid = "981fc1ac-4748-4d09-b826-7cdcb7272a99"]
pub struct Localization(
    IndexMap<HandleId, Arc<FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>>,
);

impl Localization {
    pub fn new() -> Self {
        Localization(IndexMap::new())
    }

    pub fn handles(&self) -> impl Iterator<Item = HandleId> + '_ {
        self.0.keys().cloned()
    }

    pub fn insert<H: Into<HandleId>>(&mut self, handle: H, asset: BundleAsset) {
        self.0.insert(handle.into(), asset.0);
    }

    pub fn locales(&self) -> impl Iterator<Item = &LanguageIdentifier> {
        self.0.values().map(|bundle| bundle.locale())
    }
}

impl<'a, T, U> Content<'a, T, U> for Localization
where
    T: Copy + Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>>,
{
    #[instrument(fields(request = %request.into()), skip_all)]
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
