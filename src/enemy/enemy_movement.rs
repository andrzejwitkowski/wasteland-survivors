use bevy::prelude::*;
use crate::components::movements::movement::Movement;
use crate::components::MovementState;
use crate::enemy::enemy_components::Enemy;

pub fn init_enemy_movement(
    mut commands: Commands,
    enemy_query: Query<Entity, (With<Enemy>, Without<Movement>)>,
) {
    if let Ok(enemy) = enemy_query.single() {
        commands.entity(enemy).insert(Movement::default()).insert(MovementState::Walking);
        info!("Enemy movement initialized");
    }
}


// TO DO: Implement enemy movement
pub fn update_enemy_movement(
    enemy_query: Query<(&Enemy, &Transform, &Movement, &MovementState)>,
    time: Res<Time>,
) {
    
}