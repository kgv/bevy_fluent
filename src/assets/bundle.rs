use super::resource::load_fluent_resource;
use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use log::{error, log_enabled, warn, Level};
use serde::Deserialize;
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    str,
};
use typed_builder::TypedBuilder;
use unic_langid::LanguageIdentifier;

/// `FluentBundle` wrapper.
///
/// A collection of resources for a single locale.  
/// If locale fallback chain is empty then it is interlocale bundle.
#[derive(TypeUuid)]
#[uuid = "929113bb-9187-44c3-87be-6027fc3b7ac5"]
pub struct Bundle(FluentBundle<FluentResource, IntlLangMemoizer>);

impl Bundle {
    /// Get message content by query.
    pub fn content(&self, query: &Query) -> Option<String> {
        let message = self.get_message(&query.id)?;
        let pattern = match &query.attribute {
            None => message.value()?,
            Some(key) => message.get_attribute(key)?.value(),
        };
        let mut errors = Vec::new();
        let content = self
            .format_pattern(pattern, query.args.as_ref(), &mut errors)
            .to_string();
        if log_enabled!(Level::Error) {
            for error in errors {
                error!("{}", error);
            }
        }
        Some(content)
    }
}

impl Deref for Bundle {
    type Target = FluentBundle<FluentResource, IntlLangMemoizer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bundle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Bundle loader.
#[derive(Default)]
pub struct Loader;

impl AssetLoader for Loader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move { load_bundle(bytes, load_context).await })
    }

    fn extensions(&self) -> &[&str] {
        &["bundle", "ron"]
    }
}

async fn load_bundle<'a, 'b>(bytes: &'a [u8], load_context: &'a mut LoadContext<'b>) -> Result<()> {
    let Intermediate { locales, resources } = ron::de::from_bytes(bytes)?;
    let mut fluent_bundle = FluentBundle::new_concurrent(locales);
    let mut asset_paths = Vec::new();
    let parent = load_context.path().with_file_name("");
    for mut path in resources {
        if path.is_relative() {
            path = parent.join(path);
        }
        let bytes = load_context.read_asset_bytes(&path).await?;
        let fluent_resource = load_fluent_resource(&bytes).await?;
        if let Err(error) = fluent_bundle.add_resource(fluent_resource) {
            warn!("overriding fluent message: {:?}", error);
        }
        let asset_path = AssetPath::new(path, None);
        asset_paths.push(asset_path.clone());
    }
    load_context
        .set_default_asset(LoadedAsset::new(Bundle(fluent_bundle)).with_dependencies(asset_paths));
    // load_context.set_labeled_asset(
    //     "en-US",
    //     LoadedAsset::new(Bundle {
    //         locales,
    //         resources: handles,
    //     })
    //     .with_dependencies(asset_paths),
    // );
    Ok(())
}

/// Message content query.
#[derive(TypedBuilder)]
pub struct Query<'a> {
    #[builder(setter(into))]
    id: String,
    #[builder(default, setter(into, strip_option))]
    attribute: Option<String>,
    #[builder(default, setter(into, strip_option))]
    args: Option<FluentArgs<'a>>,
}

#[derive(Deserialize)]
struct Intermediate {
    locales: Vec<LanguageIdentifier>,
    resources: Vec<PathBuf>,
}
