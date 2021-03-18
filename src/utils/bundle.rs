use bevy::prelude::*;
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::borrow::Borrow;
use typed_builder::TypedBuilder;

/// Extension methods for [`FluentBundle`](fluent::bundle::FluentBundle)
pub trait BundleExt {
    /// Request message content
    fn content(&self, request: &Request) -> Option<String>;
}

impl<T: Borrow<FluentResource>> BundleExt for FluentBundle<T, IntlLangMemoizer> {
    fn content(&self, request: &Request) -> Option<String> {
        let message = self.get_message(&request.id)?;
        let pattern = match &request.attribute {
            None => message.value()?,
            Some(key) => message.get_attribute(key)?.value(),
        };
        let mut errors = Vec::new();
        let content = self
            .format_pattern(pattern, request.args.as_ref(), &mut errors)
            .to_string();
        error_span!("format_pattern").in_scope(|| {
            for error in errors {
                error!(%error);
            }
        });
        Some(content)
    }
}

/// Message content request
///
/// Provides access to message content according to the given components.
#[derive(TypedBuilder)]
pub struct Request<'a> {
    #[builder(setter(into))]
    id: String,
    #[builder(default, setter(into, strip_option))]
    attribute: Option<String>,
    #[builder(default, setter(into, strip_option))]
    args: Option<FluentArgs<'a>>,
}
