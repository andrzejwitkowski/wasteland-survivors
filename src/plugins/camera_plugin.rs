use bevy::prelude::*;

use crate::systems::{
    camera_system::{camera_controller, init_camera},
};
use crate::systems::plane_chunk_system::init_player_startup_tile;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera.after(init_player_startup_tile));
        app.add_systems(Update, camera_controller);
    }
}
