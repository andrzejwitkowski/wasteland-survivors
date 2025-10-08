use bevy::prelude::*;

use crate::{
    components::TileRegistry, 
    systems::{level_plane_system::spawn_default_chunk_grid, plane_chunk_system::{build_tile_registry, draw_tiles_borders, handle_chunk_clicks}}
};

pub struct TestPlanePlugin;

impl Plugin for TestPlanePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileRegistry>();
        app.add_systems(
            Startup,
            (
                spawn_default_chunk_grid,
                build_tile_registry.after(spawn_default_chunk_grid),
            )
        );
        app.add_systems(Update, draw_tiles_borders);
        app.add_systems(Update, handle_chunk_clicks);
    }
}