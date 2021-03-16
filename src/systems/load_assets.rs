use crate::resources::{Handles, Settings};
use bevy::prelude::*;
use std::path::Path;

pub(crate) fn load_assets(
    asset_server: Res<AssetServer>,
    mut handles: ResMut<Handles>,
    settings: Res<Settings>,
) {
    debug!("load assets");
    let path = Path::new(&settings.locale_folder);
    handles.0 = asset_server
        .load_folder(path)
        .expect("load `assets/locales` folder");
}
