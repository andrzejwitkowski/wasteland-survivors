use crate::enemy::enemy_system::init_enemy;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup, init_enemy
        );
    }
}
