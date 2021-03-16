use fluent::FluentError;
use fluent_syntax::parser::ParserError;
use std::{borrow::Cow, fmt::Display};
use thiserror::Error;

fn join<T: Display>(t: &[T]) -> String {
    t.iter()
        .map(|v| Cow::Owned(v.to_string()))
        .intersperse(Cow::Borrowed(", "))
        .collect()
}

/// Error.
#[derive(Error, Debug)]
pub enum Error {
    #[error("add resource: [{}]", join(_0))]
    AddResource(Vec<FluentError>),
    #[error("format pattern: [{}]", join(_0))]
    FormatPattern(Vec<FluentError>),
    #[error("parse resource: [{}]", join(_0))]
    ParseResource(Vec<ParserError>),
}
