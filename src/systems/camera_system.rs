use bevy::prelude::*;
use crate::components::{CameraController, Player};

pub fn init_camera(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {
        let camera_entity = (
            Transform::from_xyz(
                player_transform.translation.x,
                player_transform.translation.y + 5.0,
                player_transform.translation.z + 10.0,
            )
            .looking_at(player_transform.translation, Vec3::Y),
            Camera3d::default(),
            CameraController,
        );
        commands.spawn(camera_entity).insert(Name::new("Main Camera"));
        info!("Camera initialized and following player");
    } else {
        warn!("No player found to follow");
    }
}

pub fn camera_controller(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {
        for mut camera_transform in camera_query.iter_mut() {
            // Simple follow logic: set camera position relative to player
            camera_transform.translation = player_transform.translation + Vec3::new(0.0, 50.0, 10.0);
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}