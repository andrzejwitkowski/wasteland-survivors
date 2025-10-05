use bevy::prelude::*;

use crate::components::Player;

pub fn init_player(mut commands: Commands) {

    let player_entity =(
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player::default().with_debug_gizmo(Color::srgb(1.0, 0.0, 0.0), 1.0),
    );
    
    commands.spawn(
        player_entity
    ).insert(Name::new("Player"));
    info!("Player initialized");
}

pub fn draw_player(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Player)>,
) {
    if query.is_empty() {
        info!("No player found");
        return;
    }
    for (transform, player) in query.iter() {
        if let Some(debug_gizmo) = &player.debug_gizmo {
            // Draw a sphere gizmo at player position
            gizmos.sphere(
                transform.translation,
                debug_gizmo.size,
                debug_gizmo.color,
            );
            // Draw a line indicating the forward direction
            gizmos.line(
                transform.translation,
                transform.translation + Vec3::Y * 2.0,
                Color::srgb(0.0, 1.0, 0.0),
            );
            // Draw coordinate axes at player position
            gizmos.ray(
                transform.translation,
                transform.forward() * 2.0,
                Color::srgb(0.0, 0.0, 1.0),
            );
            gizmos.ray(
                transform.translation,
                transform.right() * 2.0,
                Color::srgb(1.0, 0.0, 0.0),
            );
            gizmos.ray(
                transform.translation,
                transform.up() * 2.0,
                Color::srgb(0.0, 1.0, 0.0),
            );
        } else {
            info!("No debug gizmo for player");
        }
    }
}