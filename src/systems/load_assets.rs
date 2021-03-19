use crate::components::{Handles, Settings};
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};
use std::path::Path;

#[instrument(fields(handles = ?*handles, locales_folder = %settings.locales_folder), skip(asset_server, settings))]
pub(crate) fn load_assets(
    asset_server: Res<AssetServer>,
    mut handles: ResMut<Handles>,
    settings: Res<Settings>,
) {
    trace!("call");
    let path = Path::new(&settings.locales_folder);
    handles.0 = asset_server
        .load_folder(path)
        .expect("load assets from root locales directory");
}
