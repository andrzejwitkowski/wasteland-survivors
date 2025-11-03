use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::components::Tile;
use crate::components::movements::a_star_movement::AStarNode;
use bevy::prelude::*;
use crate::shared::CharacterType;

fn heuristic(pos1: Vec3, pos2: Vec3) -> f32 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    dx * dx + dy * dy + dz * dz
}

/// Diablo-style A* pathfinding.
/// Jednostki blokują tylko kafelki na których stoją (occupied_tiles).
pub fn astar_pathfind(
    start: Entity,
    goal: Entity,
    tiles: &Query<(&Tile, &Transform), Without<CharacterType>>,
    occupied_tiles: &HashSet<Entity>, // Zajęte kafelki przez inne jednostki
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

    // KLUCZOWE: Jeśli goal jest zajęty, znajdź najbliższy wolny kafelek
    let actual_goal = if occupied_tiles.contains(&goal) {
        // Goal zajęty - znajdź najbliższy sąsiedni wolny kafelek
        find_nearest_free_neighbor(goal, occupied_tiles, tiles)?
    } else {
        goal
    };

    let goal_pos = tiles.get(actual_goal).ok()?.1.translation;

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut g_scores = HashMap::new();
    let mut came_from: HashMap<Entity, Entity> = HashMap::new();

    g_scores.insert(start, 0.0);

    let h_start = heuristic(start_transform.translation, goal_pos);

    open_set.push(AStarNode { entity: start, f_score: h_start, g_score: 0.0 });

    while let Some(current_node) = open_set.pop() {
        let current = current_node.entity;

        // Goal reached (actual_goal, nie oryginalny goal!)
        if current == actual_goal {
            return Some(reconstruct_path(&came_from, current));
        }

        // Skip if already processed
        if closed_set.contains(&current) {
            continue;
        }
        closed_set.insert(current);

        // Skip if we've found a better path already
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

                // DIABLO-STYLE: Skip non-walkable tiles
                if !neighbor_tile.walkable {
                    continue;
                }

                // DIABLO-STYLE: Skip ALL occupied tiles - no exceptions!
                if occupied_tiles.contains(neighbor) {
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

                    open_set.push(AStarNode {
                        entity: *neighbor,
                        f_score,
                        g_score: tentative_g_score,
                    });
                }
            }
        }
    }

    // No path found
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

/// Znajduje najbliższy wolny kafelek sąsiadujący z goal.
/// Używane gdy goal jest zajęty - jednostka zatrzyma się obok.
fn find_nearest_free_neighbor(
    goal: Entity,
    occupied_tiles: &HashSet<Entity>,
    tiles: &Query<(&Tile, &Transform), Without<CharacterType>>,
) -> Option<Entity> {
    let (goal_tile, goal_transform) = tiles.get(goal).ok()?;
    let goal_pos = goal_transform.translation;

    // Sprawdź wszystkich sąsiadów goal
    let mut best_neighbor = None;
    let mut best_distance = f32::INFINITY;

    for maybe_neighbor in goal_tile.neighbor_entities.iter() {
        if let Some(neighbor) = maybe_neighbor {
            // Sprawdź czy sąsiad jest wolny i walkable
            if !occupied_tiles.contains(neighbor) {
                if let Ok((neighbor_tile, neighbor_transform)) = tiles.get(*neighbor) {
                    if neighbor_tile.walkable {
                        // Wybierz najbliższego (może być wiele wolnych sąsiadów)
                        let distance = heuristic(neighbor_transform.translation, goal_pos);
                        if distance < best_distance {
                            best_distance = distance;
                            best_neighbor = Some(*neighbor);
                        }
                    }
                }
            }
        }
    }

    if best_neighbor.is_some() {
        info!("Goal {:?} is occupied, redirecting to neighbor {:?}", goal, best_neighbor);
    }

    best_neighbor
}

