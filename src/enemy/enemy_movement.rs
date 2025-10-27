use bevy::prelude::*;
use crate::components::movements::movement::{MoveRequestEvent, Movement, MovementSpeed, MovementType};
use crate::components::{MovementState, TilePosition};
use crate::components::player::player::Player;
use crate::enemy::enemy_components::{Enemy, EnemyLastMovementTime};

pub fn update_enemy_movement(
    mut enemies: Query<(Entity, &MovementState, &mut EnemyLastMovementTime, &TilePosition), With<Enemy>>,
    player: Query<(&TilePosition), With<Player>>,
    mut movement_request_writer: MessageWriter<MoveRequestEvent>,
    time: Res<Time>,
) {
    if let Ok(player_tile) = player.single() {
        if let Some(player_tile_entity) = player_tile.tile {
            for (entity, movement_state, mut last_movement_time, source_tile) in enemies.iter_mut() {
                info!("Enemy movement state: {:?}", movement_state);
                // if movement_state == &MovementState::Walking {
                    if (time.elapsed_secs() - last_movement_time.time) > 1.0 {
                        movement_request_writer.write(MoveRequestEvent {
                            entity,
                            movement_type: MovementType::ASTAR,
                            source_tile_entity: source_tile.tile.unwrap(),
                            target_tile_entity: player_tile_entity,
                        });
                        last_movement_time.time = time.elapsed_secs();
                    }
                // }
            }
        }
    }
}