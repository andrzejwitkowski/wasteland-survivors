use crate::enemy::enemy_system::{draw_enemy_gizmo, init_enemy};
use bevy::prelude::*;
use crate::enemy::enemy_movement::init_enemy_movement;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, (init_enemy, init_enemy_movement, draw_enemy_gizmo)
        );
    }
}
