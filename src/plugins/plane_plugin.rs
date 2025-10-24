use bevy::prelude::*;

use crate::{
    components::TileRegistry,
    systems::{
        level_plane_system::spawn_default_chunk_grid,
        plane_chunk_system::{build_tile_registry, handle_optimized_grid_clicks},
    },
};
use crate::components::TileRegistryCreatedEvent;
use crate::systems::player::init_player_startup_tile;

pub struct TestPlanePlugin;

impl Plugin for TestPlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TileRegistryCreatedEvent>();
        app.init_resource::<TileRegistry>();
        app.add_systems(
            Startup,
            (
                spawn_default_chunk_grid,
                build_tile_registry.after(spawn_default_chunk_grid),
                init_player_startup_tile.after(build_tile_registry),
            ),
        );
        // app.add_systems(Update, draw_tiles_borders);
        app.add_systems(Update, handle_optimized_grid_clicks);
    }
}
