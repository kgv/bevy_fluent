use bevy::prelude::*;
use bevy_fluent::{components::Snapshot, utils::BundleExt, FluentPlugin, Request};

pub fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .add_system(localized_hello_world.system())
        .run();
}

fn localized_hello_world(snapshot: Option<Res<Snapshot>>) {
    if let Some(snapshot) = snapshot {
        let request = Request::builder().id("hello-world").build();
        let hello_world = snapshot.content(&request).unwrap();
        println!("{}", hello_world);
    }
}
