use crate::enemy::enemy_movement::{update_enemy_movement};
use crate::enemy::enemy_system::{draw_enemy_gizmo, init_enemy};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_enemy, draw_enemy_gizmo, update_enemy_movement),
        );
    }
}
