#![allow(clippy::type_complexity)]

use crate::{
    components::Font,
    locales::{de, en, ru},
    resources::Locales,
    systems::{load, menu},
};
use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_fluent::prelude::*;

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            asset_folder: "examples/ui/assets".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .insert_resource(Locale::new(ru::RU).with_default(en::US))
        .insert_resource(Locales(vec![de::DE, en::US, ru::BY, ru::RU]))
        .init_resource::<Font>()
        .add_state(GameState::Load)
        .add_system_set(SystemSet::on_enter(GameState::Load).with_system(load::setup))
        .add_system_set(SystemSet::on_update(GameState::Load).with_system(load::load))
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu::setup))
        .add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(menu::interaction)
                .with_system(menu::next)
                .with_system(menu::previous),
        )
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(menu::cleanup))
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Load,
    Menu,
}

mod components;
mod locales;
mod resources;
mod systems;
mod to_sentence_case;
