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
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            asset_folder: "examples/ui/assets".to_string(),
            ..default()
        }))
        .add_plugin(FluentPlugin)
        .insert_resource(Locale::new(ru::RU).with_default(en::US))
        .insert_resource(Locales(vec![de::DE, en::US, ru::BY, ru::RU]))
        .init_resource::<Font>()
        .add_state::<GameState>()
        .add_systems((
            load::setup.in_schedule(OnEnter(GameState::Load)),
            load::update.in_set(OnUpdate(GameState::Load)),
        ))
        // TODO: [nested tuples of systems](https://github.com/bevyengine/bevy/issues/7880)
        .add_systems((
            menu::setup.in_schedule(OnEnter(GameState::Menu)),
            menu::cleanup.in_schedule(OnExit(GameState::Menu)),
        ))
        .add_systems(
            (menu::interaction, menu::next, menu::previous).in_set(OnUpdate(GameState::Menu)),
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
