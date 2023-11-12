//! Bundles resource

use crate::BundleAsset;
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
};
use unic_langid::LanguageIdentifier;

/// Bundles resource
///
/// Collection of [`BundleAsset`]s.
#[derive(Clone, Default, Resource)]
pub struct Bundles(pub IndexMap<Handle<BundleAsset>, BundleAsset>);

impl Bundles {
    pub fn handles(&self) -> impl Iterator<Item = &Handle<BundleAsset>> {
        self.0.keys()
    }

    fn locales(&self) -> impl Iterator<Item = &LanguageIdentifier> {
        self.0.values().map(|bundle| &bundle.locales[0])
    }
}

impl<'a, T, U> Content<'a, T, U> for Bundles
where
    T: Copy + Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>>,
{
    #[instrument(fields(request = %request.into()), skip_all)]
    fn content(&self, request: T) -> Option<String> {
        self.0.values().find_map(|bundle| {
            let content = bundle.content(request);
            trace!(locale = %bundle.locales[0], ?content);
            content
        })
    }
}

impl Debug for Bundles {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Bundles")
            .field(&self.locales().collect::<Vec<_>>())
            .finish()
    }
}

impl FromIterator<(Handle<BundleAsset>, BundleAsset)> for Bundles {
    fn from_iter<T: IntoIterator<Item = (Handle<BundleAsset>, BundleAsset)>>(iter: T) -> Self {
        Self(FromIterator::from_iter(iter))
    }
}
