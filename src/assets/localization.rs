use crate::{
    exts::{bundle::Request, BundleExt},
    BundleAsset, ResourceAsset,
};
use bevy::{
    ecs::world::WorldCell,
    prelude::*,
    reflect::TypeUuid,
    utils::tracing::{self, instrument},
};
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
#[cfg(feature = "format")]
use format::{ext::FormatterExt, lazy_format};
use indexmap::IndexSet;
use intl_memoizer::concurrent::IntlLangMemoizer;
#[cfg(feature = "format")]
use std::fmt::{self, Display, Formatter};
use std::{borrow::Borrow, sync::Arc};
use unic_langid::LanguageIdentifier;

/// Collection of [`FluentBundle`](fluent::bundle::FluentBundle)s
#[derive(TypeUuid)]
#[uuid = "981fc1ac-4748-4d09-b826-7cdcb7272a99"]
pub struct Localization {
    bundles: Vec<FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>,
}

impl Localization {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn locales(&self) -> impl Iterator<Item = Option<&LanguageIdentifier>> {
        self.bundles.iter().map(|bundle| bundle.locales.first())
    }
}

impl<'a, T, U> BundleExt<'a, T, U> for Localization
where
    T: Copy + Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>>,
{
    #[instrument(fields(request = %request.into()), skip(self))]
    fn content(&self, request: T) -> Option<String> {
        self.bundles.iter().find_map(|bundle| {
            let content = bundle.content(request);
            #[cfg(feature = "format")]
            trace!(
                locale = %lazy_format!(|f| f.display_option(bundle.locales.first())),
                ?content,
            );
            #[cfg(not(feature = "format"))]
            trace!(locale = ?bundle.locales.first(), ?content);
            content
        })
    }
}

#[cfg(feature = "format")]
impl Display for Localization {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.display_tuple("Localization")
            .field(&lazy_format!(|f| {
                let mut display_list = f.display_list();
                for bundle in &self.bundles {
                    display_list.entry(&lazy_format!(|f| f.display_option(bundle.locales.first())));
                }
                display_list.finish()
            }))
            .finish()
    }
}

/// Localization builder
#[derive(Clone, Debug, Default)]
pub struct Builder {
    handles: IndexSet<Handle<BundleAsset>>,
}

impl Builder {
    pub fn add_handle(&mut self, handle: Handle<BundleAsset>) {
        self.handles.insert(handle);
    }

    pub fn add_handles(&mut self, handles: IndexSet<Handle<BundleAsset>>) {
        self.handles.extend(handles);
    }

    pub fn with_handle(mut self, handle: Handle<BundleAsset>) -> Self {
        self.add_handle(handle);
        self
    }

    pub fn with_handles(mut self, handles: IndexSet<Handle<BundleAsset>>) -> Self {
        self.add_handles(handles);
        self
    }

    pub fn build(self, world: &WorldCell) -> Localization {
        let bundle_assets = world.get_resource::<Assets<BundleAsset>>().unwrap();
        let resource_assets = world.get_resource::<Assets<ResourceAsset>>().unwrap();
        let mut bundles = Vec::new();
        for bundle_handle in self.handles {
            let bundle_asset = bundle_assets.get(bundle_handle).unwrap();
            let locales = bundle_asset.locale().into_iter().cloned().collect();
            let mut bundle = FluentBundle::new_concurrent(locales);
            for resource_handle in bundle_asset.resources() {
                if let Some(resource_asset) = resource_assets.get(resource_handle) {
                    if let Err(errors) = bundle.add_resource(resource_asset.0.clone()) {
                        warn_span!("add_resource").in_scope(|| {
                            for error in errors {
                                warn!(%error);
                            }
                        });
                    }
                }
            }
            bundles.push(bundle);
        }
        Localization { bundles }
    }
}
