#![allow(clippy::type_complexity)]

use crate::{
    locales::{de, en, ru},
    resources::{Font, Locales},
    systems::{load, menu},
};
use bevy::prelude::*;
use bevy_fluent::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "examples/ui/assets".to_string(),
                ..default()
            }),
            FluentPlugin,
        ))
        .insert_resource(Locale::new(ru::RU).with_default(en::US))
        .insert_resource(Locales(vec![de::DE, en::US, ru::BY, ru::RU]))
        .init_resource::<Font>()
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Load), load::setup)
        .add_systems(Update, load::update.run_if(in_state(GameState::Load)))
        .add_systems(OnEnter(GameState::Menu), menu::setup)
        .add_systems(OnExit(GameState::Menu), menu::cleanup)
        .add_systems(
            Update,
            (menu::interaction, menu::next, menu::previous).run_if(in_state(GameState::Menu)),
        )
        .run();
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Load,
    Menu,
}

mod components;
mod locales;
mod resources;
mod systems;
mod to_sentence_case;
