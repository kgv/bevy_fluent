# Documentation

[🔼](../README.md) | **English** 🇺🇸 | [Русский 🇷🇺](ru-RU.md)

## Use

Load asset using `AssetServer`:

```rust
let handle = asset_server.load("locales/en-US/main.ftl.ron");
```

Load all assets matching the glob using `AssetServerExt`:

```rust
use bevy_fluent::exts::bevy::AssetServerExt;

let handles = asset_server.load_glob("locales/**/main.ftl.ron")?;
```

Check assets load state:

```rust
if let LoadState::Loaded =  asset_server.get_load_state(handle) {
    ...
}
```

Check assets load state:

```rust
if let LoadState::Loaded = asset_server.get_group_load_state(handles) {
    ...
}
```

Create a bundle fallback chain based on the locale fallback chain using
`LocalizationBuilder`:

```rust
let localization = localization_builder.build(handles);
```

Request content:

```rust
let hello_world = bundle_asset.content("hello-world")?;
let hello_world = localization.content("hello-world")?;
```

## Definitions

[***Localization***][localization] is a Fluent [***bundle***][fluent-bundle]
fallback chain.

[***Bundle asset***][bundle-asset] - is an abstraction for presentation Fluent
*bundles*. Each *bundle asset* file has the extension `.ftl.ron`.

[***Resource asset***][resource-asset] - is an abstraction for presentation
Fluent [***resources***][fluent-resource]. Each *resource asset* file has the
extension `.ftl`. *Resource asset* is the atomic unit of disk storage for
Fluent.

Each *resource asset* is a set of [***messages***][message]. *Message* is the
basic atomic translation unit for Fluent.

Each *message* has an [***identifier***][identifier].

*Messages* (and [***terms***][term], [***variants***][variant],
[***attributes***][attribute]) store their values as [***patterns***][pattern].

Formated *pattern* are called [***content***][content].

[***Request***][request] is a request to receive *content* specified by the
parameters.

[attribute]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Attribute.html
[bundle-asset]: https://docs.rs/bevy_fluent/*/bevy_fluent/assets/struct.BundleAsset.html
[content]: https://docs.rs/bevy_fluent/*/bevy_fluent/exts/bundle/trait.BundleExt.html#tymethod.content
[fluent-bundle]: https://docs.rs/fluent/*/fluent/bundle/struct.FluentBundle.html
[fluent-resource]: https://docs.rs/fluent/*/fluent/struct.FluentResource.html
[identifier]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Identifier.html
[localization]: https://docs.rs/bevy_fluent/*/bevy_fluent/assets/struct.Localization.html
[message]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Message.html
[pattern]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Pattern.html
[request]: https://docs.rs/bevy_fluent/*/bevy_fluent/exts/bundle/struct.Request.html
[resource-asset]: https://docs.rs/bevy_fluent/*/bevy_fluent/assets/struct.ResourceAsset.html
[term]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Term.html
[unicode-language-identifier]: http://unicode.org/reports/tr35/#Unicode_language_identifier
[variant]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Variant.html
