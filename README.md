# Bevy fluent

[![continuous integration](https://github.com/kgv/bevy_fluent/workflows/CI/badge.svg)](https://github.com/kgv/bevy_fluent/actions)
[![crates.io](https://img.shields.io/crates/v/bevy_fluent.svg)](https://crates.io/crates/bevy_fluent)
[![docs.rs](https://docs.rs/bevy_fluent/badge.svg)](https://docs.rs/bevy_fluent)
[![license](https://img.shields.io/crates/l/bevy_fluent)](#license)

Bevy plugin for localization using Fluent.

## Definitions

Fluent:

The basic unit of translation in Fluent is called a ***message***. Each message
has an ***identifier***. *Messages* (and terms, variants, attributes) store
their values as ***patterns***.

Local:

Formated *pattern* are called ***content***. ***Query*** provides access to
*content* according to the given components.
