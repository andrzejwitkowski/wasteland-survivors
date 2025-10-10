use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use bevy::prelude::*;
use crate::components::{Player, PlayerMoveRequestEvent, Tile, TileSelectedEvent};
use crate::components::movement::Node;

pub fn tile_selected_event_handle(
    mut tile_selected_events: MessageReader<TileSelectedEvent>,
    player_query: Query<&Player>,
    mut player_move_events: MessageWriter<PlayerMoveRequestEvent>,
) {
    if let Ok(_) = player_query.single() {
        for event in tile_selected_events.read() {
            player_move_events.write(PlayerMoveRequestEvent {
                source_tile_entity: event.source_tile_entity,
                target_tile_entity: event.target_tile_entity,
            });
        }
    } else {
        warn!("No player found to move");
    }
}

pub fn player_movement_request_handler(
    mut player_move_events: MessageReader<PlayerMoveRequestEvent>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    tiles: Query<(&Tile, &Transform), Without<Player>>
) {
    for event in player_move_events.read() {
        if let Ok((transform, mut player)) = player_query.single_mut() {

            if player.tile_entity.is_none() {
                player.tile_entity = Some(event.source_tile_entity);
            }

            // Perform A* pathfinding from source to target tile
            if let Some(path) = astar_pathfind(event.source_tile_entity, event.target_tile_entity, &tiles) {

                info!("Path found with {} steps", path.len());
                
                player.path = VecDeque::from(path); 
                player.segment_start = transform.translation;
                player.translation_progress = 0.0;
                
            } else {
                warn!("No path found from source to target tile");
                continue;
            }
        } else {
            warn!("No player found to execute movement");
        }
    }
}

pub fn update_player_movement(
    mut query: Query<(&mut Transform, &mut Player)>,
    transforms: Query<&Transform, Without<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut query {
        // 1. If target is None and list is not empty, pop first element
        if player.target_transform.is_none() && !player.path.is_empty() {
            if let Some(next_entity) = player.path.pop_front() {
                if let Ok(target) = transforms.get(next_entity) {
                    player.segment_start = transform.translation; // ✅ Save current position
                    player.target_transform = Some(*target);
                    player.translation_progress = 0.0;
                    player.tile_entity = Some(next_entity); // Update current tile
                }
            }
        }
        
        // 2. Check if translation_progress >= 1.0
        if player.translation_progress >= 1.0 {
            // Set target to None (will pop next element on next frame)
            player.target_transform = None;
            continue;
        }
        
        // 3. Move toward target
        if let Some(target) = player.target_transform {
            player.translation_progress += time.delta_secs() * player.speed;
            player.translation_progress = player.translation_progress.min(1.0);
            
            // ✅ Lerp from segment_start to target
            transform.translation = player.segment_start.lerp(
                target.translation,
                player.translation_progress
            );
            
            // ✅ ROTATION: Face the target
            let direction = target.translation - player.segment_start;
            if direction.length_squared() > 0.001 {
                // Look at target (assumes Y-up, character faces Z-forward)
                transform.look_to(direction, Vec3::Y);
            }
        }
    }
}

fn heuristic(pos1: Vec3, pos2: Vec3) -> f32 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    dx * dx + dy * dy + dz * dz
}

pub fn astar_pathfind(
    start: Entity,
    goal: Entity,
    tiles: &Query<(&Tile, &Transform), Without<Player>>,
) -> Option<Vec<Entity>> {
    let (start_tile, start_transform) = tiles.get(start).ok()?;
    let (goal_tile, goal_transform) = tiles.get(goal).ok()?;
    
    // Check if start and goal are walkable
    if !start_tile.walkable || !goal_tile.walkable {
        return None;
    }
    
    if start == goal {
        return Some(vec![start]);
    }

    let goal_pos = goal_transform.translation;
    
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut g_scores = HashMap::new();
    let mut came_from: HashMap<Entity, Entity> = HashMap::new();
    
    info!("Starting A* from {:?} to {:?}", start, goal);
    g_scores.insert(start, 0.0);
    
    let h_start = heuristic(start_transform.translation, goal_pos);

    open_set.push(Node {
        entity: start,
        f_score: h_start,
        g_score: 0.0,
    });
    
    while let Some(current_node) = open_set.pop() {
        let current = current_node.entity;
        
        // Goal reached
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }
        
        // Skip if already processed
        if closed_set.contains(&current) {
            continue;
        }
        closed_set.insert(current);
        
        // Skip if we've found a better path already (handles duplicates in heap)
        if current_node.g_score > *g_scores.get(&current).unwrap_or(&f32::INFINITY) {
            continue;
        }
        
        let (current_tile, current_tile_transform) = match tiles.get(current) {
            Ok(tile) => tile,
            Err(_) => continue,
        };
        
        let current_pos = current_tile_transform.translation;
        let current_g_score = current_node.g_score;
        
        // Check all neighbors
        for maybe_neighbor in current_tile.neighbor_entities.iter() {
            if let Some(neighbor) = maybe_neighbor {
                // Skip if already in closed set
                if closed_set.contains(&neighbor) {
                    continue;
                }
                
                let (neighbor_tile, neighbour_tile_transform) = match tiles.get(*neighbor) {
                    Ok(tile) => tile,
                    Err(_) => continue,
                };
                
                // IMPORTANT: Skip non-walkable tiles
                if !neighbor_tile.walkable {
                    continue;
                }
                
                let neighbor_pos = neighbour_tile_transform.translation;
                let edge_cost = heuristic(current_pos, neighbor_pos);
                let tentative_g_score = current_g_score + edge_cost;
                
                // Only proceed if this path is better
                let existing_g_score = g_scores.get(&neighbor).copied().unwrap_or(f32::INFINITY);
                
                if tentative_g_score < existing_g_score {
                    came_from.insert(*neighbor, current);
                    g_scores.insert(*neighbor, tentative_g_score);
                    
                    let h_score = heuristic(neighbor_pos, goal_pos);
                    let f_score = tentative_g_score + h_score;
                    
                    open_set.push(Node {
                        entity: *neighbor,
                        f_score,
                        g_score: tentative_g_score,
                    });
                }
            }
        }
    }
    
    // No path found
    info!("No path found");
    None
}

fn reconstruct_path(came_from: &HashMap<Entity, Entity>, mut current: Entity) -> Vec<Entity> {
    let mut path = vec![current];
    
    while let Some(&previous) = came_from.get(&current) {
        current = previous;
        path.push(current);
    }
    
    path.reverse();
    path
}