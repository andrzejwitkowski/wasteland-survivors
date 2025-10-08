use bevy::prelude::*;

use crate::systems::{level_plane_system::spawn_default_chunk_grid, plane_chunk_system::{draw_tiles_borders, handle_chunk_clicks}};

pub struct TestPlanePlugin;

impl Plugin for TestPlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_default_chunk_grid);
        app.add_systems(Update, draw_tiles_borders);
        app.add_systems(Update, handle_chunk_clicks);
    }
}