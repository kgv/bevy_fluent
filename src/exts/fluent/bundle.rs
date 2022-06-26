use fluent::bundle::FluentBundle;
use unic_langid::LanguageIdentifier;

/// Extension methods for [`FluentBundle`](fluent::bundle::FluentBundle)
pub trait BundleExt {
    /// Bundle locale
    fn locale(&self) -> &LanguageIdentifier;
}

impl<R, M> BundleExt for FluentBundle<R, M> {
    fn locale(&self) -> &LanguageIdentifier {
        &self.locales[0]
    }
}
