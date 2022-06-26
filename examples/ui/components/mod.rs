use bevy::prelude::{Font as BevyFont, *};

/// Font
pub struct Font(pub Handle<BevyFont>);

impl FromWorld for Font {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        Self(font)
    }
}

/// Menu
#[derive(Component)]
pub struct Menu;

/// Next button
#[derive(Component)]
pub struct NextButton;

/// Previous button
#[derive(Component)]
pub struct PreviousButton;
