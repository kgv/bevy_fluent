use crate::components::{Handles, Settings};
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};
use std::path::Path;

#[instrument(fields(locales_folder = %settings.locales_folder), skip(commands, asset_server, settings))]
pub(crate) fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    trace!("call");
    let path = Path::new(&settings.locales_folder);
    let handles = asset_server
        .load_folder(path)
        .expect("load assets from root locales directory");
    commands.insert_resource(Handles(handles));
}
