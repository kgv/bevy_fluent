use crate::{components::Queue, BundleAsset, Localization};
use bevy::{asset::LoadState, prelude::*};

// [???](https://github.com/bevyengine/bevy/blob/d119c1ce14da59089a65373d59715a41d05251ad/crates/bevy_audio/src/audio_output.rs#L72)
pub(crate) fn serve(world: &mut World) {
    let world = world.cell();
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let bundle_assets = world.get_resource::<Assets<BundleAsset>>().unwrap();
    let mut localization_assets = world.get_resource_mut::<Assets<Localization>>().unwrap();
    let queue = world.get_resource_mut::<Queue>().unwrap();
    let mut writer = queue.write();
    for index in 0..writer.len() {
        if let Some((id, handles)) = writer.pop_front() {
            if localization_assets.contains(id) {
                continue;
            }
            let bundle_handles = handles.iter().map(|handle| handle.id);
            let load_state = asset_server.get_group_load_state(bundle_handles);
            trace!(%index, ?id, ?load_state);
            if load_state == LoadState::Loaded {
                let resource_handles = handles
                    .iter()
                    .map(|handle| {
                        let bundle_asset = bundle_assets.get(handle).unwrap();
                        bundle_asset.resources()
                    })
                    .flatten()
                    .map(|handle| handle.id);
                if asset_server.get_group_load_state(resource_handles) == LoadState::Loaded {
                    let localization = Localization::builder().with_handles(handles).build(&world);
                    let _ = localization_assets.set(id, localization);
                    continue;
                }
            }
            writer.push_back((id, handles));
        }
    }
}
