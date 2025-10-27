use std::collections::VecDeque;

use crate::components::movements::movement::{MoveRequestEvent, Movement, MovementSpeed, MovementType};
use crate::components::player::player::Player;
use crate::components::{ModelAnimationGraph, MovementState, Tile, TilePosition, TileSelectedEvent};
use crate::systems::movement::a_star_movement::astar_pathfind;
use bevy::prelude::*;
use crate::shared::CharacterType;

pub fn init_player_movement(
    mut commands: Commands,
    player_query: Query<Entity, (With<Player>, Without<Movement>)>,
) {
    if let Ok(player) = player_query.single() {
        commands.entity(player).insert(Movement::default()).insert(MovementState::Idle);
        info!("Player movement initialized");
    }
}

pub fn tile_selected_event_handle(
    mut tile_selected_events: MessageReader<TileSelectedEvent>,
    player_query: Query<(Entity), With<Player>>,
    mut player_move_events: MessageWriter<MoveRequestEvent>,
) {
    if let Ok(entity) = player_query.single() {
        for event in tile_selected_events.read() {
            player_move_events.write(MoveRequestEvent {
                entity,
                movement_type: MovementType::ASTAR,
                source_tile_entity: event.source_tile_entity,
                target_tile_entity: event.target_tile_entity,
            });
        }
    } else {
        warn!("No player found to move");
    }
}

pub fn movement_request_handler(
    mut move_events: MessageReader<MoveRequestEvent>,
    mut character_query: Query<(&mut Transform, &mut Movement, &mut TilePosition, &mut MovementState), With<CharacterType>>,
    tiles: Query<(&Tile, &Transform), Without<CharacterType>>,
) {
    for event in move_events.read() {
        if let Ok((transform, mut player_movement, mut tile_position, mut movement_state)) = character_query.get_mut(event.entity) {
            if tile_position.tile.is_none() {
                tile_position.tile = Some(event.source_tile_entity);
            }

            if !player_movement.path.is_empty() {
                if let Some(current_target) = player_movement.path.back() {
                    if *current_target == event.target_tile_entity {
                        info!("Already moving to this tile, ignoring duplicate click");
                        continue;
                    }
                }
                info!("Interrupting current path for new destination");
            }

            if let Some(tile_entity) = tile_position.tile {

                let paths = match event.movement_type {
                    MovementType::ASTAR => astar_pathfind(tile_entity, event.target_tile_entity, &tiles),
                    MovementType::SHORTEST => None
                };

                if let Some(path) = paths {
                    info!("New path found with {} steps", path.len());

                    player_movement.path = VecDeque::from(path);
                    player_movement.segment_start = transform.translation; // Start from current position
                    player_movement.translation_progress = 0.0;
                    player_movement.target_transform = None; // THIS IS THE KEY LINE FOR INTERRUPTION
                    player_movement.segment_distance = 0.0;

                    *movement_state = MovementState::Walking;

                } else {
                    warn!("No path found to target tile");
                }
            }
        } else {
            warn!("No player found to execute movement");
        }
    }
}

pub fn update_player_movement(
    query: Query<(&mut Transform, &mut MovementSpeed, &mut Movement, &mut TilePosition, &mut MovementState, &CharacterType)>,
    transforms: Query<&Transform, Without<MovementSpeed>>,
    time: Res<Time>,
) {
    for (mut transform, speed, mut movement, mut tile_position, mut movement_state, character_type) in query {
        if movement.target_transform.is_none() && !movement.path.is_empty() {
            if let Some(next_entity) = movement.path.pop_front() {
                if let Ok(target) = transforms.get(next_entity) {
                    movement.segment_start = transform.translation; // ✅ Save current position
                    movement.target_transform = Some(*target);
                    movement.translation_progress = 0.0;
                    tile_position.tile = Some(next_entity); // Update current tile

                    movement.segment_distance =
                        movement.segment_start.distance(target.translation);

                    // ✅ ROTATION: Face the target
                    let direction = target.translation - movement.segment_start;
                    if direction.length_squared() > 0.001 {
                        // Look at target (assumes Y-up, character faces Z-forward)
                        transform.look_to(-direction.normalize(), Vec3::Y);
                    }
                }
            }
        }

        // 2. Check if translation_progress >= 1.0
        if movement.translation_progress >= 1.0 {
            // Set target to None (will pop next element on next frame)
            movement.target_transform = None;
            continue;
        }

        // 3. Move toward target
        if let Some(target) = movement.target_transform {
            let movement_this_frame = speed.speed * time.delta_secs();
            let progress_increment = if movement.segment_distance > 0.0 {
                movement_this_frame / movement.segment_distance
            } else {
                1.0 // Instant movement for zero distance
            };

            movement.translation_progress += progress_increment;
            movement.translation_progress = movement.translation_progress.min(1.0);

            // ✅ Lerp from segment_start to target
            transform.translation = movement
                .segment_start
                .lerp(target.translation, movement.translation_progress);
        }

        if character_type == &CharacterType::Player &&movement.path.is_empty() && *movement_state != MovementState::Idle {
            *movement_state = MovementState::Idle;
        }
    }
}
