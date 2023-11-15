#![allow(clippy::type_complexity)]

use crate::{
    resources::Font,
    systems::{load, menu},
};
use bevy::prelude::*;
use bevy_fluent::prelude::*;
use resources::Locales;
use unic_langid::langid;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "examples/ui/assets".to_string(),
                ..default()
            }),
            FluentPlugin,
        ))
        .insert_resource(Locales(vec![
            langid!("de-DE"),
            langid!("en-US"),
            langid!("ru-BY"),
            langid!("ru-RU"),
        ]))
        .init_resource::<Font>()
        .add_state::<GameState>()
        .add_systems(OnEnter(GameState::Load), load::setup)
        .add_systems(Update, load::update.run_if(in_state(GameState::Load)))
        .add_systems(OnEnter(GameState::Menu), menu::setup)
        .add_systems(
            Update,
            (menu::interaction, menu::next, menu::previous).run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), menu::cleanup)
        .run();
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Load,
    Menu,
}

mod components;
mod resources;
mod systems;
mod to_sentence_case;
