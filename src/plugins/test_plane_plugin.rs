use bevy::prelude::*;

use crate::systems::test_plane_system::{draw_tiles_borders, init_test_plane, handle_chunk_clicks};

pub struct TestPlanePlugin;

impl Plugin for TestPlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_test_plane);
        app.add_systems(Update, draw_tiles_borders);
        app.add_systems(Update, handle_chunk_clicks);
    }
}