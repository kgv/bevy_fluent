# Bevy fluent

[![crates.io](https://img.shields.io/crates/v/bevy_fluent.svg)](https://crates.io/crates/bevy_fluent)
[![docs.rs](https://docs.rs/bevy_fluent/badge.svg)](https://docs.rs/bevy_fluent)
[![license](https://img.shields.io/crates/l/bevy_fluent)](#license)
[![ci](https://github.com/kgv/bevy_fluent/workflows/ci/badge.svg)](https://github.com/kgv/bevy_fluent/actions)
[![bevy tracking](https://img.shields.io/badge/bevy%20tracking-main-lightblue)](https://github.com/bevyengine/bevy/blob/master/docs/plugins_guidelines.md#master-branch-tracking)

Bevy plugin for localization using Fluent.

## `Settings`

- `default_locale`
- `locale_folder`
- `requested_locales`

## Definitions

Fluent:

- each `*.ftl` file is a resource.

The basic unit of translation in Fluent is called a ***message***. Each
*message* has an ***identifier***. *Messages* (and ***terms***, ***variants***,
***attributes***) store their values as ***patterns***.

Local:

Formated *pattern* are called ***content***. ***Query*** provides access to
*content* according to the given components.

## Locales root directory hierarchy (`locales`)

***Locale*** is a collection of `*.ftl` files. *Locale* represents a directory
that meets the following criteria: name matches
[standard][unicode_language_identifier], contains a `locale.ron` file. The
`locale.ron` file contains a list of `*.ftl` files related to this *locale*. The
files themselves are located in the hierarchy of the *locale* root directory.
The `locale.ron` file, located in the `locales` root directory, represents an
interlocale that contains language independent resources.

Hierarchy example:

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

Note that `locales/ru` directory does not contain the `locale.ron` file, so it
is not *locale* directory. However, you can create a `locales/ru/locale.ron`
file, which will add `ru` locale.

## Todo

- [ ] refactor assets retrieving using
  [`get_handle_path`](https://docs.rs/bevy/0.4.0/bevy/asset/struct.AssetServer.html#method.get_handle_path)
  ([pr](https://github.com/bevyengine/bevy/pull/1290)),

## See Also

- [0.4 -> 0.5 Migration Guide](https://github.com/bevyengine/bevy/issues/1601)

- [Issue](https://github.com/bevyengine/bevy/issues/461)
- [Discussions](https://github.com/bevyengine/bevy/discussions/1345)

- [Fluent fallback](https://crates.io/crates/fluent_fallback)
- [Fluent resource manager](https://crates.io/crates/fluent_resmgr)
- [Message format 2.0`](https://github.com/zbraniecki/message-format-2.0-rs)
- [L10nRegistry](https://github.com/zbraniecki/l10nregistry-rs)

- [Locales](https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-core/availableLocales.json)
- [Unicode Language Identifier][unicode_language_identifier]

[unicode_language_identifier]: http://unicode.org/reports/tr35/#Unicode_language_identifier
