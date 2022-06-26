# Документация

[🔼](../README.md) | [English 🇺🇸](en-US.md) | **Русский** 🇷🇺

## Использование

Загрузить ассет с помощью `AssetServer`:

```rust
let handle = asset_server.load("locales/ru-RU/main.ftl.ron");
```

Загрузить все ассеты, удовлетворяющие шаблону, с помощью `AssetServerExt`:

```rust
use bevy_fluent::exts::bevy::AssetServerExt;

let handles = asset_server.load_glob("locales/**/main.ftl.ron")?;
```

Проверить статус загрузки ассета:

```rust
if let LoadState::Loaded =  asset_server.get_load_state(handle) {
    ...
}
```

Проверить статус загрузки нескольких ассетов:

```rust
if let LoadState::Loaded = asset_server.get_group_load_state(handles) {
    ...
}
```

Создать резервную цепочку бандлов на основе резервной цепочки локалей с помощью
`LocalizationBuilder`:

```rust
let localization = localization_builder.build(handles);
```

Запросить контент:

```rust
let hello_world = bundle_asset.content("hello-world")?;
let hello_world = localization.content("hello-world")?;
```

## Определения

[***Локализация***][localization] представляет собой резервную цепочку
[***бандлов***][fluent-bundle] Fluent.

[***Ассет бандла***][bundle-asset] - является абстракцией для представления
*бандлов* Fluent. Файл *ассета бандла* имеет расширение `.ftl.ron`.

[***Ассет ресурса***][resource-asset] - является абстракцией для представления
[***ресурсов***][fluent-resource] Fluent. Файл *ассета ресурсов* имеет
расширение `.ftl`. *Ассет ресурса* является атомарной единицей хранения
информации на диске для Fluent.

Каждый *ассет ресурса* представляет собой набор [***сообщений***][message].
*Cообщение* является атомарной единицей перевода во Fluent.

Каждое *сообщение* имеет [***идентификатор***][identifier].

*Сообщения* (как и [***термы***][term], [***варианты***][variant],
[***аттрибуты***][attribute]) хранят свои значения в виде
[***паттернов***][pattern].

Форматированный *паттерн* называется [***контентом***][content].

[***Запрос***][request] представляет собой запрос на получение соответствующего
заданным параметрам *контента*.

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
