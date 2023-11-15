use bevy::prelude::{Font as BevyFont, *};
use bevy_fluent::BundlesAsset;
use unic_langid::LanguageIdentifier;

/// Font
#[derive(Resource)]
pub struct Font(pub(crate) Handle<BevyFont>);

impl FromWorld for Font {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        Self(font)
    }
}

/// Loading
#[derive(Resource)]
pub struct Loading(pub(crate) Handle<BundlesAsset>);

impl From<Loading> for AssetId<BundlesAsset> {
    fn from(value: Loading) -> Self {
        value.into()
    }
}

/// Loaded
#[derive(Resource)]
pub struct Loaded(pub(crate) Handle<BundlesAsset>);

/// Locales
#[derive(Deref, Resource)]
pub struct Locales(pub(crate) Vec<LanguageIdentifier>);

impl Locales {
    pub(crate) fn next(&mut self) {
        self.0.rotate_right(1);
    }

    pub(crate) fn previous(&mut self) {
        self.0.rotate_left(1);
    }
}
