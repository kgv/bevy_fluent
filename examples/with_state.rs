use bevy::prelude::*;
use bevy_fluent::{prelude::*, utils::BundleExt, Snapshot};
use unic_langid::langid;

pub fn main() {
    App::build()
        .insert_resource(FluentSettings::default().with_default_locale(langid!("ru-RU")))
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .add_state(GameState::Initialize)
        .add_system_set(
            SystemSet::on_update(GameState::Initialize).with_system(check_fluent_state.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Play).with_system(localized_hello_world.system()),
        )
        .run();
}

fn check_fluent_state(
    fluent_state: Res<State<FluentState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if *fluent_state.current() == FluentState::Done {
        game_state.overwrite_set(GameState::Play).unwrap();
    }
}

fn localized_hello_world(snapshot: Res<Snapshot>, mut done: Local<bool>) {
    if *done {
        return;
    }
    *done = true;
    let request = Request::builder().id("hello-world").build();
    let hello_world = snapshot.content(&request).unwrap();
    println!("{}", hello_world);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum GameState {
    Initialize,
    Play,
}
