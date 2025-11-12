mod components;
mod materials;
mod plugins;
mod systems;
mod enemy;
mod shared;
mod player;
mod level;

use bevy::app::App;
use bevy::prelude::*;
use bevy_landmass::Landmass3dPlugin;
use bevy_rerecast::{Mesh3dBackendPlugin, NavmeshPlugins};
use bevy_rts_camera::RtsCameraPlugin;
use bevy_skein::SkeinPlugin;
use landmass_rerecast::LandmassRerecastPlugin;
use crate::enemy::EnemyPlugin;
use crate::level::gltf_level_plugin::GltfLevelPlugin;
use crate::materials::pavement::CheckedFloorMaterials;
use crate::plugins::{CameraPlugin, PlayerPlugin, TestPlanePlugin, TileSelectionPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NavmeshPlugins::default())
        .add_plugins(Mesh3dBackendPlugin::default())
        .add_plugins(Landmass3dPlugin::default())
        .add_plugins(LandmassRerecastPlugin::default())
        .add_plugins(SkeinPlugin::default())
        .init_resource::<CheckedFloorMaterials>()
        .add_plugins(RtsCameraPlugin)
        .add_plugins(GltfLevelPlugin)
        // .add_plugins(MeshPickingPlugin)
        // .add_plugins(PlayerPlugin)
        // .add_plugins(EnemyPlugin)
        // .add_plugins(TestPlanePlugin)
        // .add_plugins(CameraPlugin)
        // .add_plugins(TileSelectionPlugin)
        .run();
}
