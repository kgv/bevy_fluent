use std::path::{Component, Path, StripPrefixError};
use thiserror::Error;

fn iter_after_stem<'a, 'b, I, J>(mut iter: I, stem: J) -> Option<I>
where
    I: Iterator<Item = Component<'a>> + Clone,
    J: Iterator<Item = Component<'b>> + Clone,
{
    loop {
        let mut stem_outer = stem.clone();
        let mut iter_outer = iter.clone();
        let mut iter_next = iter_outer.next();
        let mut iter_inner = iter_outer.clone();
        loop {
            match (iter_next, stem_outer.next()) {
                (Some(x), Some(y)) if x == y => {}
                (Some(_), Some(_)) => break,
                (None, Some(_)) => return None,
                (_, None) => return Some(iter),
            }
            iter = iter_inner.clone();
            iter_next = iter_inner.next();
        }
        iter = iter_outer;
    }
}

fn iter_before_stem<'a, 'b, I, J>(mut iter: I, stem: J) -> Option<I>
where
    I: DoubleEndedIterator<Item = Component<'a>> + Clone,
    J: DoubleEndedIterator<Item = Component<'b>> + Clone,
{
    loop {
        let mut stem_outer = stem.clone();
        let mut iter_outer = iter.clone();
        let mut iter_next = iter_outer.next_back();
        let mut iter_inner = iter_outer.clone();
        loop {
            match (iter_next, stem_outer.next_back()) {
                (Some(x), Some(y)) if x == y => {}
                (Some(_), Some(_)) => break,
                (None, Some(_)) => return None,
                (_, None) => return Some(iter),
            }
            iter = iter_inner.clone();
            iter_next = iter_inner.next_back();
        }
        iter = iter_outer;
    }
}

fn iter_before_suffix<'a, 'b, I, J>(mut iter: I, mut suffix: J) -> Option<I>
where
    I: DoubleEndedIterator<Item = Component<'a>> + Clone,
    J: DoubleEndedIterator<Item = Component<'b>>,
{
    loop {
        let mut iter_inner = iter.clone();
        match (iter_inner.next_back(), suffix.next_back()) {
            (Some(ref x), Some(ref y)) if x == y => {}
            (_, Some(_)) => return None,
            (_, None) => return Some(iter),
        }
        iter = iter_inner;
    }
}

/// Extension methods for [`Path`](std::path::Path)
pub trait PathExt {
    fn find_prefix<P>(&self, predicate: P) -> Result<&Path, PrefixError>
    where
        P: FnMut(&Self) -> bool;

    fn prefix<P: AsRef<Path>>(&self, stem: P) -> Result<&Path, PrefixError>;

    fn strip<P, Q>(&self, prefix: P, suffix: Q) -> Result<&Path, StripError>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>;

    fn strip_suffix<P: AsRef<Path>>(&self, suffix: P) -> Result<&Path, StripSuffixError>;

    fn suffix<P: AsRef<Path>>(&self, stem: P) -> Result<&Path, SuffixError>;
}

impl PathExt for Path {
    fn find_prefix<P>(&self, mut predicate: P) -> Result<&Path, PrefixError>
    where
        P: FnMut(&Self) -> bool,
    {
        let mut components = self.components();
        loop {
            if predicate(components.as_path()) {
                return Ok(components.as_path());
            }
            components.next_back().ok_or(PrefixError(()))?;
        }
    }

    fn prefix<P: AsRef<Path>>(&self, stem: P) -> Result<&Path, PrefixError> {
        iter_before_stem(self.components(), stem.as_ref().components())
            .map(|components| components.as_path())
            .ok_or(PrefixError(()))
    }

    fn strip<P, Q>(&self, prefix: P, suffix: Q) -> Result<&Path, StripError>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        Ok(self.strip_prefix(prefix)?.strip_suffix(suffix)?)
    }

    fn strip_suffix<P: AsRef<Path>>(&self, suffix: P) -> Result<&Path, StripSuffixError> {
        iter_before_suffix(self.components(), suffix.as_ref().components())
            .map(|components| components.as_path())
            .ok_or(StripSuffixError(()))
    }

    fn suffix<P: AsRef<Path>>(&self, stem: P) -> Result<&Path, SuffixError> {
        iter_after_stem(self.components(), stem.as_ref().components())
            .map(|components| components.as_path())
            .ok_or(SuffixError(()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrefixError(());

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum StripError {
    #[error(transparent)]
    StripPrefixError(#[from] StripPrefixError),
    #[error(transparent)]
    StripSuffixError(#[from] StripSuffixError),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("suffix not found")]
pub struct StripSuffixError(());

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuffixError(());

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let path = Path::new("mods/mod_name/locales/*/main/bundle.ron");
        assert_eq!(Ok(Path::new("mods/mod_name/locales")), path.prefix("*"));
        assert_eq!(Ok(Path::new("main/bundle.ron")), path.suffix("*"));
        let path = Path::new("mods/mod_name/locales/**/main/bundle.ron");
        assert_eq!(Ok(Path::new("mods/mod_name/locales")), path.prefix("**"));
        assert_eq!(Ok(Path::new("main/bundle.ron")), path.suffix("**"));
    }

    mod prefix {
        use super::*;

        #[test]
        fn relative() {
            let path = Path::new("foo/bar/baz/qux.txt");
            assert_eq!(Ok(Path::new("")), path.prefix("foo"));
            assert_eq!(Ok(Path::new("foo")), path.prefix("bar"));
            assert_eq!(Ok(Path::new("foo/bar")), path.prefix("baz"));
            assert_eq!(Ok(Path::new("foo/bar/baz")), path.prefix("qux.txt"));
            assert_eq!(Ok(Path::new("")), path.prefix("foo/bar"));
            assert_eq!(Ok(Path::new("foo")), path.prefix("bar/baz"));
            assert_eq!(Ok(Path::new("foo/bar")), path.prefix("baz/qux.txt"));
            assert_eq!(Ok(Path::new("")), path.prefix("foo/bar/baz"));
            assert_eq!(Ok(Path::new("foo")), path.prefix("bar/baz/qux.txt"));
            assert_eq!(Ok(Path::new("")), path.prefix("foo/bar/baz/qux.txt"));
        }
    }

    mod strip_suffix {
        use super::*;

        #[test]
        fn absolute() {
            let path = Path::new("/foo/bar/baz.txt");
            assert_eq!(Ok(Path::new("/foo/bar/baz.txt")), path.strip_suffix(""));
            assert_eq!(Ok(Path::new("/foo/bar")), path.strip_suffix("baz.txt"));
            assert_eq!(Ok(Path::new("/foo")), path.strip_suffix("bar/baz.txt"));
            assert_eq!(Ok(Path::new("/")), path.strip_suffix("foo/bar/baz.txt"));
            assert_eq!(Ok(Path::new("")), path.strip_suffix("/foo/bar/baz.txt"));
            assert!(matches!(path.strip_suffix("baz"), Err(_)));
            assert!(matches!(path.strip_suffix("bar/baz"), Err(_)));
            assert!(matches!(path.strip_suffix("/foo/bar/baz"), Err(_)));
            assert!(matches!(path.strip_suffix("/foo/bar"), Err(_)));
            assert!(matches!(path.strip_suffix("/foo"), Err(_)));
            assert!(matches!(path.strip_suffix("/baz.txt"), Err(_)));
            assert!(matches!(path.strip_suffix("/bar/baz.txt"), Err(_)));
        }

        #[test]
        fn relative() {
            let path = Path::new("foo/bar/baz.txt");
            assert_eq!(Ok(Path::new("foo/bar/baz.txt")), path.strip_suffix(""));
            assert_eq!(Ok(Path::new("foo/bar")), path.strip_suffix("baz.txt"));
            assert_eq!(Ok(Path::new("foo")), path.strip_suffix("bar/baz.txt"));
            assert_eq!(Ok(Path::new("")), path.strip_suffix("foo/bar/baz.txt"));
            assert!(matches!(path.strip_suffix("baz"), Err(_)));
            assert!(matches!(path.strip_suffix("bar/baz"), Err(_)));
            assert!(matches!(path.strip_suffix("foo/bar/baz"), Err(_)));
            assert!(matches!(path.strip_suffix("foo/bar"), Err(_)));
            assert!(matches!(path.strip_suffix("foo"), Err(_)));
            assert!(matches!(path.strip_suffix("/baz.txt"), Err(_)));
            assert!(matches!(path.strip_suffix("/bar/baz.txt"), Err(_)));
            assert!(matches!(path.strip_suffix("/foo/bar/baz.txt"), Err(_)));
        }
    }

    mod suffix {
        use super::*;

        #[test]
        fn relative() {
            let path = Path::new("foo/bar/baz/qux.txt");
            assert_eq!(Ok(Path::new("bar/baz/qux.txt")), path.suffix("foo"));
            assert_eq!(Ok(Path::new("baz/qux.txt")), path.suffix("bar"));
            assert_eq!(Ok(Path::new("qux.txt")), path.suffix("baz"));
            assert_eq!(Ok(Path::new("")), path.suffix("qux.txt"));
            assert_eq!(Ok(Path::new("baz/qux.txt")), path.suffix("foo/bar"));
            assert_eq!(Ok(Path::new("qux.txt")), path.suffix("bar/baz"));
            assert_eq!(Ok(Path::new("")), path.suffix("baz/qux.txt"));
            assert_eq!(Ok(Path::new("qux.txt")), path.suffix("foo/bar/baz"));
            assert_eq!(Ok(Path::new("")), path.suffix("bar/baz/qux.txt"));
            assert_eq!(Ok(Path::new("")), path.suffix("foo/bar/baz/qux.txt"));
        }
    }
}
