use bevy::prelude::*;
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::borrow::Borrow;
use typed_builder::TypedBuilder;

pub trait BundleExt {
    /// Get message content by query.
    fn content(&self, query: &Query) -> Option<String>;
}

impl<T: Borrow<FluentResource>> BundleExt for FluentBundle<T, IntlLangMemoizer> {
    fn content(&self, query: &Query) -> Option<String> {
        let message = self.get_message(&query.id)?;
        let pattern = match &query.attribute {
            None => message.value()?,
            Some(key) => message.get_attribute(key)?.value(),
        };
        let mut errors = Vec::new();
        let content = self
            .format_pattern(pattern, query.args.as_ref(), &mut errors)
            .to_string();
        error_span!("format_pattern").in_scope(|| {
            for error in errors {
                error!(%error);
            }
        });
        Some(content)
    }
}

/// Message content query.
///
/// Provides access to message content according to the given components.
#[derive(TypedBuilder)]
pub struct Query<'a> {
    #[builder(setter(into))]
    id: String,
    #[builder(default, setter(into, strip_option))]
    attribute: Option<String>,
    #[builder(default, setter(into, strip_option))]
    args: Option<FluentArgs<'a>>,
}
