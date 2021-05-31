use crate::{
    components::{ColorMaterials, DefaultFont, Locales},
    locales::{de, en, ru},
    systems::{load, menu},
};
use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_fluent::prelude::*;

fn main() {
    App::build()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::ERROR,
            filter: "bevy_fluent=trace".to_string(),
        })
        .insert_resource(AssetServerSettings {
            asset_folder: "examples/ui/assets".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .insert_resource(
            Locales::new(de::DE)
                .with_default(en::US)
                .with(ru::BY)
                .with(ru::RU),
        )
        .init_resource::<ColorMaterials>()
        .init_resource::<DefaultFont>()
        .add_state(GameState::Load)
        .add_system_set(SystemSet::on_enter(GameState::Load).with_system(load::setup.system()))
        .add_system_set(SystemSet::on_update(GameState::Load).with_system(load::loading.system()))
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu::setup.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(menu::interaction.system())
                .with_system(menu::next.system())
                .with_system(menu::previous.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(menu::cleanup.system()))
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Load,
    Menu,
}

mod components;
mod locales;
mod pathfinders;
mod systems;
mod to_sentence_case;
