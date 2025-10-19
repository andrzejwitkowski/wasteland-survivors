use std::collections::VecDeque;

use crate::components::movements::movement::{MoveRequestEvent, Movement, MovementType};
use crate::components::player::player::Player;
use crate::components::{ModelAnimationGraph, Tile, TilePosition, TileSelectedEvent};
use crate::systems::movement::a_star_movement::astar_pathfind;
use bevy::prelude::*;

pub fn init_player_movement(
    mut commands: Commands,
    player_query: Query<Entity, (With<Player>, Without<Movement>)>,
) {
    if let Ok(player) = player_query.single() {
        commands.entity(player).insert(Movement::default());
        info!("Player movement initialized");
    }
}

pub fn tile_selected_event_handle(
    mut tile_selected_events: MessageReader<TileSelectedEvent>,
    player_query: Query<&Player>,
    mut player_move_events: MessageWriter<MoveRequestEvent>,
) {
    if let Ok(_) = player_query.single() {
        for event in tile_selected_events.read() {
            player_move_events.write(MoveRequestEvent {
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
    mut player_move_events: MessageReader<MoveRequestEvent>,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Movement, &mut TilePosition, &ModelAnimationGraph)>,
    tiles: Query<(&Tile, &Transform), Without<Player>>,
) {
    for event in player_move_events.read() {
        if let Ok((transform, mut player, mut player_movement, mut tile_position, model_animation)) = player_query.single_mut() {
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
    mut query: Query<(&mut Transform, &mut Player, &mut Movement, &mut TilePosition)>,
    transforms: Query<&Transform, Without<Player>>,
    time: Res<Time>,
) {
    for (mut transform, player, mut player_movement, mut tile_position) in &mut query {
        if player_movement.target_transform.is_none() && !player_movement.path.is_empty() {
            if let Some(next_entity) = player_movement.path.pop_front() {
                if let Ok(target) = transforms.get(next_entity) {
                    player_movement.segment_start = transform.translation; // ✅ Save current position
                    player_movement.target_transform = Some(*target);
                    player_movement.translation_progress = 0.0;
                    tile_position.tile = Some(next_entity); // Update current tile

                    player_movement.segment_distance =
                        player_movement.segment_start.distance(target.translation);

                    // ✅ ROTATION: Face the target
                    let direction = target.translation - player_movement.segment_start;
                    if direction.length_squared() > 0.001 {
                        // Look at target (assumes Y-up, character faces Z-forward)
                        transform.look_to(-direction.normalize(), Vec3::Y);
                    }
                }
            }
        }

        // 2. Check if translation_progress >= 1.0
        if player_movement.translation_progress >= 1.0 {
            // Set target to None (will pop next element on next frame)
            player_movement.target_transform = None;
            continue;
        }

        // 3. Move toward target
        if let Some(target) = player_movement.target_transform {
            let movement_this_frame = player.speed * time.delta_secs();
            let progress_increment = if player_movement.segment_distance > 0.0 {
                movement_this_frame / player_movement.segment_distance
            } else {
                1.0 // Instant movement for zero distance
            };

            player_movement.translation_progress += progress_increment;
            player_movement.translation_progress = player_movement.translation_progress.min(1.0);

            // ✅ Lerp from segment_start to target
            transform.translation = player_movement
                .segment_start
                .lerp(target.translation, player_movement.translation_progress);
        }
    }
}
