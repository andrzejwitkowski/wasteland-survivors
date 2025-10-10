mod components;
mod systems;
mod plugins;
mod materials;

use bevy::prelude::*;
use bevy::app::App;

use crate::plugins::{CameraPlugin, PlayerPlugin, TestPlanePlugin, TileSelectionPlugin};
use crate::materials::pavement::CheckedFloorMaterials;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CheckedFloorMaterials>()
        .add_plugins(MeshPickingPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(TestPlanePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TileSelectionPlugin)
        .run();
}