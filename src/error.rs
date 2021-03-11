use fluent::FluentError;
use fluent_syntax::parser::ParserError;
use thiserror::Error;

/// Result.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error.
#[derive(Error, Debug)]
pub enum Error {
    #[error("parse resource ({:?})", _0)]
    ParseResource(Vec<ParserError>),
    #[error("add resource ({:?})", _0)]
    AddResource(Vec<FluentError>),
}
