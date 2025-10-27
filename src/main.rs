mod components;
mod materials;
mod plugins;
mod systems;
mod enemy;
mod shared;

use bevy::app::App;
use bevy::prelude::*;
use crate::enemy::EnemyPlugin;
use crate::materials::pavement::CheckedFloorMaterials;
use crate::plugins::{CameraPlugin, PlayerPlugin, TestPlanePlugin, TileSelectionPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CheckedFloorMaterials>()
        .add_plugins(MeshPickingPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(TestPlanePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TileSelectionPlugin)
        .run();
}
