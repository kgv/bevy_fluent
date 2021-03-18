# Bevy fluent

[![continuous integration](https://github.com/kgv/bevy_fluent/workflows/CI/badge.svg)](https://github.com/kgv/bevy_fluent/actions)
[![crates.io](https://img.shields.io/crates/v/bevy_fluent.svg)](https://crates.io/crates/bevy_fluent)
[![docs.rs](https://docs.rs/bevy_fluent/badge.svg)](https://docs.rs/bevy_fluent)
[![license](https://img.shields.io/crates/l/bevy_fluent)](#license)

Bevy плагин для локализации, использующий Fluent.

## Настройки (`Settings`)

- `default_locale`
- `locale_folder`
- `requested_locales`

## Определения

Fluent:

- each `*.ftl` file is a resource.

Атомарная единица перевода во Fluent называется ***сообщение*** (***message***).
Каждое *сообщение* имеет ***идентификатор*** (***identifier***). ***Сообщения***
(как и ***термы*** (***terms***), ***варианты*** (***variants***),
***аттрибуты*** (***attributes***)) хранят свои значения в виде ***паттернов***
(***patterns***).

Локальные:

***Локаль*** - это коллекция `*.ftl` файлов.

Каждый файл `*.ftl` представляет собой ***ресурс*** (***Resource***).

Форматированный *паттерн* называется ***контентом*** (***content***).

***Request*** предоставляет собой запрос *контента* соответствующего заданным
компонентам.

## Ресурс `Settings`

## Иерархия корневой директории локалей (`locales`)

`bevy_fluent` поддерживает два режима сканирования корневой директории:
***явный*** и ***неявный***. По умолчанию используется *явный* режим.
Переключение режима осуществляется с помощью фичи `implicit`.

### Явный режим

В этом режиме *локаль* представляет собой директорию, удовлетворяющую следующим
критериям: имя соответствует [стандарту][unicode_language_identifier], содержит
файл `locale.ron`. Файл `locale.ron` содержит список `*.ftl` файлов, относящихся
к этой *локали*. Сами файлы располагаются в иерархии корневой директории данной
*локали*. Файл `locale.ron`, расположенный в корневой директории локалей
`locales` представляет интерлокаль, которая содержит ресурсы, не зависящие от
языка.

На заметку: файлы `locale.ron` располагаются по аналогии с `mod.rs` в Rust.

Пример иерархии:

```md
locales
    - en-US
        locale.ron
        ...
    - ru
        - ru-RU
            locale.ron
            ...
        - ru-BY
            locale.ron
            ...
    locale.ron
```

Заметьте, что `locales/ru` не содержит файл `locale.ron`, поэтому не считается
директорией *локали*. Однако вы можете создать файл `locales/ru/locale.ron`, в
результате чего будет добавлена дополнительная локаль `ru`.

### Неявный режим

В этом режиме директория или файл на глубине 0 является *локалью*, если ее имя
соответствует [стандарту][unicode_language_identifier]. Директории или файлы на
глубине 0, не соответствующие указанному стандарту принадлежат интерлокали.
Директория или файл на глубине больше 0 является *локалью*, если родительская
директория также является *локалью*, и родительская *локаль* является ее
надмножеством. Файлы `*.ftl`, расположенные в иерархии корневой директории
соответствующей *локали* относятся к этой *локали*.

Пример иерархии:

```md
locales
    - en-US
        ...
    - ru
        - ru-RU
            ...
        - ru-BY
            ...
    locale.ron
```

## See Also

- [0.4 -> 0.5 Migration Guide](https://github.com/bevyengine/bevy/issues/1601)

- [Issue](https://github.com/bevyengine/bevy/issues/461)
- [Discussions](https://github.com/bevyengine/bevy/discussions/1345)

- [Fluent fallback](https://crates.io/crates/fluent_fallback)
- [Fluent resource manager](https://crates.io/crates/fluent_resmgr)
- [Message format 2.0`](https://github.com/zbraniecki/message-format-2.0-rs)
- [L10nRegistry](https://github.com/zbraniecki/l10nregistry-rs)

- [Locales](https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-core/availableLocales.json)
- [Unicode Language Identifiers][unicode_language_identifier]

[unicode_language_identifier]: http://unicode.org/reports/tr35/#Unicode_language_identifier
