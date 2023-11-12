use bevy::prelude::{Font as BevyFont, *};
use bevy_fluent::BundleAsset;

/// Font
#[derive(Resource)]
pub struct Font(pub Handle<BevyFont>);

impl FromWorld for Font {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        Self(font)
    }
}

/// Handles
#[derive(Clone, Default, Deref, Resource)]
pub struct Handles(pub Vec<Handle<BundleAsset>>);
