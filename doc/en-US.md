# Documentation

[🔼](../README.md) | **English** 🇺🇸 | [Русский 🇷🇺](ru-RU.md)

## Use

Load asset using `AssetServer`:

```rust
let handle = asset_server.load("locales/.ftl.ron#en-US");
```

Check assets load state:

```rust
if let LoadState::Loaded =  asset_server.get_load_state(handle) {
    ...
}
```

Request content:

```rust
let hello_world = bundle_asset.content("hello-world")?;
```

## File formats

- `ResourceAsset` have to be `FTL`.
- `BundlesAsset` files may be `YAML` or `RON`

## Definitions
[`BundleAsset`][bundle-asset] - is an abstraction for presentation
[`FluentBundle`][fluent-bundle]. A *bundles* file has the extension `.ftl.ron`
or `.ftl.yml` and proper format. It contains information about all
`FluentBundle`s.

[`ResourceAsset`][resource-asset] - is an abstraction for presentation
[`FluentResource`][fluent-resource]. A *resource* file has the extension `.ftl`.

Each `ResourceAsset` is a set of [`Message`][message]s. `Message` is the basic
atomic translation unit for Fluent.

Each `Message` has an [`Identifier`][identifier].

`Message`s (and [`Term`][Term]s, [`Variant`][variant]s,
[`Attribute`][attribute]s) store their values as [`Pattern`][pattern]s.

Formated `Pattern` are called [`Content`][content].

[`Request`][request] is a request to receive `Content` specified by the
parameters.

[attribute]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Attribute.html
[bundle-asset]: https://docs.rs/bevy_fluent/*/bevy_fluent/assets/struct.BundleAsset.html
[content]: https://docs.rs/bevy_fluent/*/bevy_fluent/exts/bundle/trait.BundleExt.html#tymethod.content
[fluent-bundle]: https://docs.rs/fluent/*/fluent/bundle/struct.FluentBundle.html
[fluent-resource]: https://docs.rs/fluent/*/fluent/struct.FluentResource.html
[identifier]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Identifier.html
[message]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Message.html
[pattern]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Pattern.html
[request]: https://docs.rs/bevy_fluent/*/bevy_fluent/exts/bundle/struct.Request.html
[resource-asset]: https://docs.rs/bevy_fluent/*/bevy_fluent/assets/struct.ResourceAsset.html
[term]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Term.html
[unicode-language-identifier]: http://unicode.org/reports/tr35/#Unicode_language_identifier
[variant]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Variant.html
