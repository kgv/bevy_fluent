# Документация

[🔼](../README.md) | [English 🇺🇸](en-US.md) | **Русский** 🇷🇺

## Использование

Загрузить ассет с помощью `AssetServer`:

```rust
let handle = asset_server.load("locales/.ftl.ron#ru-RU");
```

Проверить статус загрузки ассета:

```rust
if let LoadState::Loaded =  asset_server.get_load_state(handle) {
    ...
}
```

Запросить контент:

```rust
let hello_world = bundle_asset.content("hello-world")?;
```

## Определения

[`BundleAsset`][bundle-asset] - является абстракцией для представления
[`FluentBundle`][fluent-bundle]. Файл *бандлов* имеет расширение `.ftl.ron` или
`.ftl.yml` и соответствующий формат. Он содержит информацию обо всех
`FluentBundle`.

[`ResourceAsset`][resource-asset] - является абстракцией для представления
[`FluentResource`][fluent-resource]. Файл *ресурса* имеет расширение `.ftl`.

Каждый `ResourceAsset` представляет собой набор из [`Message`][message].
`Message` является атомарной единицей перевода во Fluent.

Каждое `Message` имеет [`Identifier`][identifier].

`Message` (как и [`Term`][term], [`Variant`][variant], [`Attribute`][attribute])
хранят свои значения в виде [`Pattern`][pattern].

Форматированный `Pattern` называется [`Content`][content].

[`Request`][request] представляет собой запрос на получение `Content`,
соответствующего заданным параметрам.

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
