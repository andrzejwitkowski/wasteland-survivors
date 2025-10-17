use bevy::prelude::*;

use crate::components::{gizmo::DebugGizmo, player::player::Player};

pub fn set_player_gizmo_debug(
    mut commands: Commands,
    player_query: Query<Entity, (With<Player>, Without<DebugGizmo>)>,
) {
    if let Ok(player) = player_query.single() {
        commands.entity(player).insert(DebugGizmo { ..Default::default() });
        info!("Debug gizmo added to player");
    }
}

pub fn draw_player(mut gizmos: Gizmos, query: Query<(&Transform, &DebugGizmo)>) {
    if query.is_empty() {
        info!("No player found");
        return;
    }
    for (transform, debug_gizmo) in query.iter() {
        // Draw a sphere gizmo at player position
        gizmos.sphere(transform.translation, debug_gizmo.size, debug_gizmo.color);
        // Draw a line indicating the forward direction
        gizmos.line(
            transform.translation,
            transform.translation + Vec3::Y * 2.0,
            Color::srgb(0.0, 1.0, 0.0),
        );
        // Draw coordinate axes at player position
        gizmos.ray(transform.translation, transform.forward() * 2.0, Color::srgb(0.0, 0.0, 1.0));
        gizmos.ray(transform.translation, transform.right() * 2.0, Color::srgb(1.0, 0.0, 0.0));
        gizmos.ray(transform.translation, transform.up() * 2.0, Color::srgb(0.0, 1.0, 0.0));
    }
}
