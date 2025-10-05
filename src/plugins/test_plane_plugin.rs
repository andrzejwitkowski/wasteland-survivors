use bevy::prelude::*;

use crate::systems::test_plane_system::init_test_plane;

pub struct TestPlanePlugin;

impl Plugin for TestPlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_test_plane);
    }
}