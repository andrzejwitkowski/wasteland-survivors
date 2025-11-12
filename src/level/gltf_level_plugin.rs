use bevy::prelude::*;
use crate::level::floor::basic_floor::on_add_floor_component;
use crate::level::scene::scene_loader::{load_scene, on_navmesh_ready, on_scene_loaded, NavRes};

pub struct GltfLevelPlugin;

impl Plugin for GltfLevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NavRes>();
        app.add_observer(on_add_floor_component);
        app.add_systems(Startup, load_scene);
        app.add_observer(on_scene_loaded);
        app.add_observer(on_navmesh_ready);
    }
}