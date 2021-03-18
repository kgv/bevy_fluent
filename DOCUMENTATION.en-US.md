# Documentation

[🔼](README.md) | **English** 🇺🇸 | [Русский 🇷🇺](DOCUMENTATION.ru-RU.md)

## Use

## Settings

Settings are set using the `Settings` resource. It provides the following
options:

- `default_locale` - the locale to use as the default in your application,
- `fallback_locale_chain` - the fallback locale chain you want to use in your
  application,
- `locales_folder` - the *root locales folder*.

## Definitions

***Root locales directory*** - is the root directory for all locales. By default
it is `assets/locales/`.

***Locale directory*** - is a directory for a specific locale. For example
`assets/locales/ru/`.

Fluent [***Asset***][asset] or simply *asset* - is any file that matches the
pattern `*.ftl`. *Asset* is the atomic unit of disk storage for Fluent.

[***Locale assets***][locale-assets] is a collection of *assets* associated with
a single locale.

Each asset is a set of [***messages***][message]. *Message* is the basic atomic
translation unit for Fluent.

Each *message* has an [***identifier***][identifier].

*Messages* (and [***terms***][term], [***variants***][variant],
[***attributes***][attribute]) store their values as [***patterns***][pattern].

Formated *pattern* are called [***content***][content].

[***Request***][request] provides access to *content* according to the given
components.

[***Request***][request] is a request to receive *content* specified by the
parameters.

## The *root locales directory* structure

`bevy_fluent` supports two scanning modes, and, accordingly, two variants of the
structure of the *root locales directory*: [***explicit***][explicit] and
[***implicit***][implicit]. The default is *explicit* mode. Mode switching is
carried out using the [`implicit`][implicit] feature.

### Explicit

In this mode, each *locale directory* must meet the following criteria: the
folder name corresponds to the [standard][unicode_language_identifier], the
directory contains a file `locale.ron`. The `locale.ron` file is a list of
*asset* files for the corresponding locale, in other words, it is serialized
*locale assets*. The *asset* files themselves are located in the *locale
directory* hierarchy. The `locale.ron` file located in the *root locales
directory* represents an interlocal which contains language independent
resources.

Note: `locale.ron` files are located in the same way as `mod.rs` in Rust.

Example:

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

Note that the example's snapshot will not contain the `ru` locale, since
`locales/ru/` does not contain a `locale.ron` file, therefore it is not
considered a *locale directory*. However, if you create the file
`locales/ru/locale.ron`, the `ru` locale will be added to snapshot.

### Implicit

In this mode, a directory or file at depth 0 is a *locale directory* if its name
matches the [standard][unicode_language_identifier]. Directories or files at
depth 0 that do not meeting the specified standard belong to interlocals. A
directory or file deeper than 0 is a *locale directory* if the parent directory
is also a *locale directory* and the parent directory's locale is a superset of
its locale. *Assets* located in the hierarchy of the corresponding *locale
directory* belong to that locale.

Example:

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

[asset]: https://github.com/bevyengine/bevy
[attribute]: https://github.com/bevyengine/bevy
[content]: https://github.com/bevyengine/bevy
[explicit]: https://github.com/bevyengine/bevy
[identifier]: https://github.com/bevyengine/bevy
[implicit]: https://github.com/bevyengine/bevy
[locale-assets]: https://github.com/bevyengine/bevy
[message]: https://github.com/bevyengine/bevy
[pattern]: https://github.com/bevyengine/bevy
[request]: https://github.com/bevyengine/bevy
[term]: https://github.com/bevyengine/bevy
[variant]: https://github.com/bevyengine/bevy

[unicode-language-identifier]: http://unicode.org/reports/tr35/#Unicode_language_identifier
