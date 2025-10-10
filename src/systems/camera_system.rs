use bevy::prelude::*;
use crate::components::{CameraController, Player};

#[derive(Component)]
pub struct CameraFollow {
    pub follow_speed: f32,           // Base follow speed
    pub distance_multiplier: f32,    // How much distance affects speed
    pub max_speed: f32, 
    pub offset: Vec3,                // Fixed offset from player position
}

impl Default for CameraFollow {
    fn default() -> Self {
        Self {
            follow_speed: 4.0,
            distance_multiplier: 0.5,
            max_speed: 20.0,
            offset: Vec3::new(0.0, 50.0, 10.0), // Fixed camera position relative to player
        }
    }
}

pub fn init_camera(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {

        let camera_offset = Vec3::new(0.0, 50.0, 10.0);
        let camera_position = player_transform.translation + camera_offset;

        let camera_entity = (
            Transform::from_translation(camera_position)
                .looking_at(player_transform.translation, Vec3::Y),
            Camera3d::default(),
            CameraFollow {
                offset: camera_offset,
                ..Default::default()
            },
            CameraController,
        );
        commands.spawn(camera_entity).insert(Name::new("Main Camera"));
        info!("Camera initialized and following player");
    } else {
        warn!("No player found to follow");
    }
}

pub fn camera_controller(
    mut camera_query: Query<(&mut Transform, &CameraFollow), (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (mut camera_transform, camera_follow) in camera_query.iter_mut() {
            let desired_position = player_transform.translation + camera_follow.offset;
            
            // Calculate distance from current position to desired position
            let displacement = desired_position - camera_transform.translation;
            let distance = displacement.length();
            
            // Calculate speed based on distance (rubber band effect)
            let target_speed = camera_follow.follow_speed + (distance * camera_follow.distance_multiplier);
            let speed = target_speed.min(camera_follow.max_speed);
            
            // Move camera towards desired position (position only, rotation stays fixed)
            if distance > 0.01 {
                let move_amount = speed * time.delta_secs();
                let t = (move_amount / distance).min(1.0);
                camera_transform.translation = camera_transform.translation.lerp(desired_position, t);
            }
        }
    }
}
