use bevy::prelude::*;

use crate::systems::{camera_system::{camera_controller, init_camera}, draw_player_system::init_player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera.after(init_player));
        app.add_systems(Update, camera_controller);
    }
}

