use std::collections::HashMap;
use std::hash::Hash;
use crate::{
    components::{PlaneChunk, Tile, TileRegistry, TileSelectedEvent, player::player::Player},
    materials::pavement,
};
use bevy::prelude::*;
use crate::components::TilePosition;

/// Spawns a grid of plane chunks arranged in columns and rows
///
/// # Arguments
/// * `num_cols` - Number of columns in the grid
/// * `num_rows` - Number of rows in the grid
/// * `chunk_width` - Width of each individual chunk
/// * `chunk_height` - Height of each individual chunk
/// * `grid_size` - Number of tiles per chunk side
pub fn spawn_single_chunk_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    pavement_materials: &mut ResMut<pavement::CheckedFloorMaterials>,
    col: i32,
    row: i32,
    num_cols: i32,
    num_rows: i32,
    chunk_width: i32,
    chunk_height: i32,
    grid_size: i32,
) {
    info!("Spawning optimized chunk grid: {}x{} chunks", num_cols, num_rows);

    // Calculate chunk position
    let x_pos = (col as f32 * chunk_width as f32) - ((num_cols as f32 * chunk_width as f32) / 2.0)
        + (chunk_width as f32 / 2.0);
    let z_pos = (row as f32 * chunk_height as f32)
        - ((num_rows as f32 * chunk_height as f32) / 2.0)
        + (chunk_height as f32 / 2.0);

    // Create the plane chunk
    let plane_chunk = PlaneChunk {
        x: col,
        z: row,
        width: chunk_width,
        height: chunk_height,
        color: Color::srgb(0.0, 1.0, 0.0),
        grid_size,
    };

    let transform = Transform::from_translation(Vec3::new(x_pos, 0.0, z_pos));

    // ✅ SINGLE CLICKABLE MESH for the entire grid
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(chunk_width as f32, chunk_height as f32))),
        MeshMaterial3d(pavement_materials.material.clone()),
        transform.clone(),
        Pickable::default(),
        plane_chunk,
        Name::new(format!("Grid Mesh ({}, {})", col, row)),
    ));

    // ✅ STILL SPAWN INDIVIDUAL TILE ENTITIES for metadata
    spawn_optimized_tile_entities(commands, &plane_chunk, &transform);

    debug!("Spawned optimized grid at position ({}, {})", x_pos, z_pos);
}

fn spawn_optimized_tile_entities(
    commands: &mut Commands,
    plane_chunk: &PlaneChunk,
    chunk_transform: &Transform,
) {
    for local_z in 0..plane_chunk.grid_size {
        for local_x in 0..plane_chunk.grid_size {
            // Calculate GLOBAL coordinates
            let global_x = plane_chunk.x * plane_chunk.grid_size + local_x;
            let global_z = plane_chunk.z * plane_chunk.grid_size + local_z;

            let tile_width = plane_chunk.width as f32 / plane_chunk.grid_size as f32;
            let tile_height = plane_chunk.height as f32 / plane_chunk.grid_size as f32;

            let local_x_pos = (local_x as f32 * tile_width) - (plane_chunk.width as f32 / 2.0)
                + (tile_width / 2.0);
            let local_z_pos = (local_z as f32 * tile_height) - (plane_chunk.height as f32 / 2.0)
                + (tile_height / 2.0);

            let world_pos = chunk_transform.translation + Vec3::new(local_x_pos, 0.0, local_z_pos);

            // Spawn tile entity WITHOUT mesh - just metadata
            commands.spawn((
                Tile {
                    x: global_x, // Store GLOBAL coordinates
                    z: global_z, // Store GLOBAL coordinates
                    walkable: true,
                    selected: false,
                    hovered: false,
                    idle_color: Color::srgb(0.0, 0.0, 0.0),
                    selected_color: Color::srgb(1.0, 0.0, 0.0),
                    hovered_color: Color::srgb(0.0, 1.0, 0.0),
                    neighbor_entities: [None; 8],
                },
                Transform::from_translation(world_pos), // ✅ Pass the grid's transform
                Name::new(format!("Tile ({}, {})", global_x, global_z)),
            ));
        }
    }
}

pub fn handle_optimized_grid_clicks(
    mut click_events: MessageReader<Pointer<Click>>,
    grid_query: Query<(&Transform, &PlaneChunk)>,
    tile_registry: Res<TileRegistry>,
    mut tile_selected_events: MessageWriter<TileSelectedEvent>,
    player_query: Query<(&Transform, &Player)>,
) {
    for event in click_events.read() {
        if let Ok((grid_transform, plane_chunk)) = grid_query.get(event.entity) {
            if let Some(hit_position) = event.hit.position {
                // Convert click position to tile coordinates
                if let Some((local_x, local_z)) =
                    world_to_tile_coords(hit_position, grid_transform, plane_chunk)
                {
                    // Calculate global coordinates for TARGET tile
                    let target_global_x = plane_chunk.x * plane_chunk.grid_size + local_x;
                    let target_global_z = plane_chunk.z * plane_chunk.grid_size + local_z;

                    info!(
                        "Looking for tile at global coordinates ({}, {})",
                        target_global_x, target_global_z
                    );

                    // ✅ USE REGISTRY: O(1) lookup for target tile
                    if let Some(&target_tile_entity) =
                        tile_registry.tiles_by_coord.get(&(target_global_x, target_global_z))
                    {
                        if let Ok((player_transform, _)) = player_query.single() {
                            // ✅ CORRECT: Use new function to find player's actual tile
                            if let Some((source_global_x, source_global_z)) =
                                find_player_tile_coords(player_transform, &grid_query)
                            {
                                info!(
                                    "Looking for source tile at global coordinates ({}, {})",
                                    source_global_x, source_global_z
                                );

                                // ✅ USE REGISTRY: O(1) lookup for source tile
                                if let Some(&source_tile_entity) = tile_registry
                                    .tiles_by_coord
                                    .get(&(source_global_x, source_global_z))
                                {
                                    info!(
                                        "Found source tile entity ({}, {})",
                                        source_global_x, source_global_z
                                    );

                                    if source_tile_entity != target_tile_entity {
                                        tile_selected_events.write(TileSelectedEvent {
                                            source_tile_entity: source_tile_entity,
                                            target_tile_entity: target_tile_entity,
                                        });
                                    }
                                }
                            }
                        } else {
                            info!("Player is not found?");
                        }
                    } else {
                        info!("Something sus is happening");
                    }
                }
            }
        }
    }
}

fn find_player_tile_coords(
    player_transform: &Transform,
    grid_query: &Query<(&Transform, &PlaneChunk)>,
) -> Option<(i32, i32)> {
    for (chunk_transform, chunk) in grid_query.iter() {
        // Oblicz globalne granice chunku
        let chunk_min_x = chunk_transform.translation.x - (chunk.width as f32 / 2.0);
        let chunk_max_x = chunk_transform.translation.x + (chunk.width as f32 / 2.0);
        let chunk_min_z = chunk_transform.translation.z - (chunk.height as f32 / 2.0);
        let chunk_max_z = chunk_transform.translation.z + (chunk.height as f32 / 2.0);

        let px = player_transform.translation.x;
        let pz = player_transform.translation.z;

        if px >= chunk_min_x && px < chunk_max_x && pz >= chunk_min_z && pz < chunk_max_z {
            // Oblicz lokalne współrzędne względem chunku
            let tile_width = chunk.width as f32 / chunk.grid_size as f32;
            let tile_height = chunk.height as f32 / chunk.grid_size as f32;

            let local_x = ((px - chunk_min_x) / tile_width).floor() as i32;
            let local_z = ((pz - chunk_min_z) / tile_height).floor() as i32;

            // Oblicz globalne współrzędne
            let global_x = chunk.x * chunk.grid_size + local_x;
            let global_z = chunk.z * chunk.grid_size + local_z;
            return Some((global_x, global_z));
        }
    }
    None
}

fn world_to_tile_coords(
    world_pos: Vec3,
    grid_transform: &Transform,
    plane_chunk: &PlaneChunk,
) -> Option<(i32, i32)> {
    // Transform to grid local space using the inverse of the transform matrix
    let local_pos = grid_transform.compute_affine().inverse().transform_point3(world_pos);

    // Calculate tile size
    let tile_width = plane_chunk.width as f32 / plane_chunk.grid_size as f32;
    let tile_height = plane_chunk.height as f32 / plane_chunk.grid_size as f32;

    // Calculate grid boundaries in local space
    let grid_half_width = plane_chunk.width as f32 / 2.0;
    let grid_half_height = plane_chunk.height as f32 / 2.0;

    // Calculate which tile was clicked
    let tile_x = ((local_pos.x + grid_half_width) / tile_width).floor() as i32;
    let tile_z = ((local_pos.z + grid_half_height) / tile_height).floor() as i32;

    // Check if within this grid's bounds
    if tile_x >= 0
        && tile_x < plane_chunk.grid_size
        && tile_z >= 0
        && tile_z < plane_chunk.grid_size
    {
        Some((tile_x, tile_z))
    } else {
        None
    }
}

pub fn build_tile_registry(
    mut tile_registry: ResMut<TileRegistry>,
    tile_query: Query<(Entity, &mut Tile)>,
) {
    tile_registry.tiles_by_coord.clear();
    for (entity, tile) in tile_query.iter() {
        tile_registry.tiles_by_coord.insert((tile.x, tile.z), entity);
    }
    info!("Built TileRegistry with {} tiles", tile_registry.tiles_by_coord.len());

    calculate_tile_neighbors(&tile_registry, tile_query);
}

fn calculate_tile_neighbors(
    tile_registry: &ResMut<TileRegistry>,
    mut tile_query: Query<(Entity, &mut Tile)>,
) {
    let mut updated_count = 0;
    for (_, mut tile) in tile_query.iter_mut() {
        let old_neighbors = tile.neighbor_entities;
        tile.neighbor_entities = calculate_neighbors_for_tile(&tile_registry, tile.x, tile.z);
        if old_neighbors != tile.neighbor_entities {
            updated_count += 1;
        }
    }
    if updated_count > 0 {
        debug!("Updated neighbors for {} tiles", updated_count);
    }
}

/// Calculate neighbor entities for a specific tile coordinate
fn calculate_neighbors_for_tile(
    tile_registry: &TileRegistry,
    x: i32,
    z: i32,
) -> [Option<Entity>; 8] {
    let directions = [
        (0, 1),   // N
        (1, 1),   // NE
        (1, 0),   // E
        (1, -1),  // SE
        (0, -1),  // S
        (-1, -1), // SW
        (-1, 0),  // W
        (-1, 1),  // NW
    ];

    let mut neighbors = [None; 8];

    for (i, (dx, dz)) in directions.iter().enumerate() {
        let neighbor_coord = (x + dx, z + dz);
        if let Some(&neighbor_entity) = tile_registry.tiles_by_coord.get(&neighbor_coord) {
            neighbors[i] = Some(neighbor_entity);
        }
    }

    neighbors
}

pub fn init_player_startup_tile(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    tile_registry: Res<TileRegistry>,
    grid_query: Query<(&Transform, &PlaneChunk), Without<Player>>,
) {
    if let Some((player_entity, mut player_transform)) = player_query.single_mut().ok() {
        if let Some(middle) = get_middle(&tile_registry.tiles_by_coord) {
            commands.entity(player_entity).insert(
                TilePosition {
                    tile: Some(middle.1.clone())
                }
            );
            if let Some(world_pos) = calculate_tile_world_position(*middle.0, &grid_query) {
                player_transform.translation = world_pos;
                info!("Player positioned at middle tile: {:?}", world_pos);
            }
        }
    }
}

fn get_middle<K,V>(map: &HashMap<K,V>) -> Option<(&K, &V)>
where
    K: Clone + Ord + Hash,
    V: Clone
{
    if map.is_empty() {
        return None;
    }

    let mut keys: Vec<&K> = map.keys().collect();
    keys.sort();

    let middle_index = keys.len() / 2;
    if let Some(value) = map.get(keys[middle_index]) {
        return Some((keys[middle_index], value));
    }
    None
}

fn calculate_tile_world_position(
    tile_coord: (i32, i32),
    grid_query: &Query<(&Transform, &PlaneChunk), Without<Player>>,
) -> Option<Vec3> {
    let (tile_x, tile_z) = tile_coord;

    for (chunk_transform, chunk) in grid_query.iter() {
        // Sprawdź czy kafelek należy do tego chunku
        let local_x = tile_x - chunk.x * chunk.grid_size;
        let local_z = tile_z - chunk.z * chunk.grid_size;

        if local_x >= 0 && local_x < chunk.grid_size && local_z >= 0 && local_z < chunk.grid_size {
            // Oblicz pozycję lokalną w chunk
            let tile_width = chunk.width as f32 / chunk.grid_size as f32;
            let tile_height = chunk.height as f32 / chunk.grid_size as f32;

            let local_x_pos = (local_x as f32 * tile_width) - (chunk.width as f32 / 2.0) + (tile_width / 2.0);
            let local_z_pos = (local_z as f32 * tile_height) - (chunk.height as f32 / 2.0) + (tile_height / 2.0);

            // Przekształć na pozycję świata
            let world_pos = chunk_transform.translation + Vec3::new(local_x_pos, 0.0, local_z_pos);
            return Some(world_pos);
        }
    }

    None
}
