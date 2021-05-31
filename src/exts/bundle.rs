use bevy::prelude::*;
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource, FluentValue};
use fmt::Write;
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::{
    borrow::Borrow,
    fmt::{self, Display, Formatter},
};

fn parse_args(args: &str) -> FluentArgs {
    let mut fluent_args = FluentArgs::new();
    for arg in args.split('&') {
        match arg.split_once('=') {
            Some((key, value)) => match value {
                "" => fluent_args.set(key, FluentValue::None),
                value => fluent_args.set(key, value),
            },
            None => fluent_args.set(arg.trim_end_matches('='), FluentValue::Error),
        }
    }
    fluent_args
}

/// Extension methods for [`FluentBundle`](fluent::bundle::FluentBundle)
pub trait BundleExt<'a, T: Into<Request<'a, U>>, U: Borrow<FluentArgs<'a>>> {
    /// Request message content
    fn content(&self, request: T) -> Option<String>;
}

impl<'a, T, U, V> BundleExt<'a, T, U> for FluentBundle<V, IntlLangMemoizer>
where
    T: Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>>,
    V: Borrow<FluentResource>,
{
    fn content(&self, request: T) -> Option<String> {
        let request = request.into();
        let request = request.borrow();
        let message = self.get_message(request.id)?;
        let pattern = match request.attr {
            Some(key) => message.get_attribute(key)?.value(),
            None => message.value()?,
        };
        let mut errors = Vec::new();
        let content = self
            .format_pattern(
                pattern,
                request.args.as_ref().map(Borrow::borrow),
                &mut errors,
            )
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
/// Provides access to a message content. Attribute and arguments are optional.
///
/// # Examples
///
/// Only identifier:
///
/// ```
/// # use bevy_fluent::exts::bundle::Request;
/// #
/// let request = Request::from("id");
/// ```
///
/// Identifier and attribute:
///
/// ```
/// # use bevy_fluent::exts::bundle::Request;
/// #
/// let request = Request::from("id.attr");
/// ```
///
/// Identifier and argument:
///
/// ```
/// # use bevy_fluent::exts::bundle::Request;
/// #
/// let request = Request::from("id?key=value");
/// ```
///
/// Identifier attribute and arguments:
///
/// ```
/// # use bevy_fluent::exts::bundle::Request;
/// #
/// let request = Request::from("id.attr?key1=value1&key2=value2");
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Request<'a, T> {
    pub id: &'a str,
    pub attr: Option<&'a str>,
    pub args: Option<T>,
}

impl<'a> Request<'a, &'static FluentArgs<'static>> {
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl<'a, T> Request<'a, T> {
    pub fn attr(self, attr: &'a str) -> Self {
        Self {
            attr: Some(attr),
            ..self
        }
    }
}

impl<'a, T> Request<'a, T> {
    pub fn args<U>(self, args: U) -> Request<'a, U> {
        Request {
            id: self.id,
            attr: self.attr,
            args: Some(args),
        }
    }
}

impl<T> Default for Request<'_, T> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            attr: Default::default(),
            args: Default::default(),
        }
    }
}

impl<'a, T: Borrow<FluentArgs<'a>>> Display for Request<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r#""{}"#, self.id)?;
        if let Some(attribute) = &self.attr {
            write!(f, ".{}", attribute)?;
        }
        if let Some(args) = &self.args {
            let mut args = args.borrow().iter().peekable();
            if args.peek().is_some() {
                f.write_char('?')?;
            }
            for (key, value) in args {
                write!(f, "{}=", key)?;
                match value {
                    FluentValue::String(key) => write!(f, "{}", key)?,
                    FluentValue::Number(key) => write!(f, "{}", key.as_string())?,
                    // FluentValue::Custom(Box<dyn FluentType + Send>),
                    _ => {}
                }
            }
        }
        f.write_char('"')?;
        Ok(())
    }
}

impl<'a> From<&'a String> for Request<'a, FluentArgs<'a>> {
    fn from(value: &'a String) -> Self {
        Self::from(&**value)
    }
}

impl<'a> From<&'a str> for Request<'a, FluentArgs<'a>> {
    fn from(value: &'a str) -> Self {
        match value.split_once('.') {
            Some((id, value)) => match value.split_once('?') {
                Some((attr, args)) => Self {
                    id,
                    attr: Some(attr),
                    args: Some(parse_args(args)),
                },
                None => Self {
                    id,
                    attr: Some(value),
                    ..Default::default()
                },
            },
            None => match value.split_once('?') {
                Some((id, args)) => Self {
                    id,
                    args: Some(parse_args(args)),
                    ..Default::default()
                },
                None => Self {
                    id: value,
                    ..Default::default()
                },
            },
        }
    }
}
