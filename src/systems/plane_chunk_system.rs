use bevy::{asset::RenderAssetUsages, mesh::{Indices, PrimitiveTopology}, prelude::*};
use crate::{components::{PlaneChunk, Tile, TileRegistry}, materials::pavement};

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
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pavement_materials: &mut ResMut<pavement::CheckedFloorMaterials>,
    col: i32,
    row: i32,
    num_cols: i32,
    num_rows: i32,
    chunk_width: i32,
    chunk_height: i32,
    grid_size: i32,
) {
    info!("Spawning chunk grid: {}x{} chunks", num_cols, num_rows);
    
    // Calculate chunk position based on column and row
    let x_pos = (col as f32 * chunk_width as f32) - ((num_cols as f32 * chunk_width as f32) / 2.0) + (chunk_width as f32 / 2.0);
    let z_pos = (row as f32 * chunk_height as f32) - ((num_rows as f32 * chunk_height as f32) / 2.0) + (chunk_height as f32 / 2.0);
    
    // Create the plane chunk
    let plane_chunk = PlaneChunk {
        x: col,
        z: row,
        width: chunk_width,
        height: chunk_height,
        grid_size,
        color: Color::srgb(
            (col as f32 / num_cols as f32) * 0.5 + 0.3,
            (row as f32 / num_rows as f32) * 0.5 + 0.3,
            0.5,
        ),
        ..default()
    };
    
    // Spawn the chunk entity
    commands.spawn((
        Mesh3d(meshes.add(create_plane_chunk_mesh(&plane_chunk))),
        // MeshMaterial3d(materials.add(StandardMaterial {
        //     base_color: plane_chunk.color,
        //     perceptual_roughness: 1.0,
        //     ..default()
        // })),
        MeshMaterial3d(pavement_materials.material.clone()),
        Transform::from_xyz(x_pos, 0.0, z_pos),
        plane_chunk,
        Name::new(format!("Plane Chunk ({}, {})", col, row)),
    ));
    
    // Spawn tiles for this chunk
    spawn_tile_meshes(
        commands,
        meshes,
        materials,
        &plane_chunk,
        Transform::from_xyz(x_pos, 0.0, z_pos),
    );
    
    debug!("Spawned chunk at position ({}, {})", x_pos, z_pos);
}

fn create_plane_chunk_mesh(test_plane: &PlaneChunk) -> Mesh {
    let width = test_plane.width;
    let height = test_plane.height;
    
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    
    for z in 0..height {
        for x in 0..width {
            let u = x as f32 / (width - 1) as f32;
            let v = z as f32 / (height - 1) as f32;
            
            // Create vertices in normalized space (-0.5 to 0.5)
            let x_pos = (u - 0.5) * width as f32;
            let z_pos = (v - 0.5) * height as f32;
            
            vertices.push([x_pos, 0.0, z_pos]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([u * 1.0, v * 1.0]); // Scale UVs for better texture mapping
        }
    }
    
    // Generate indices for triangles
    for z in 0..(height - 1) {
        for x in 0..(width - 1) {
            let i = (z * width + x) as u32;
            
            // Two triangles per quad
            indices.extend_from_slice(&[
                i, i + width as u32, i + 1,
                i + 1, i + width as u32, i + width as u32 + 1,
            ]);
        }
    }
    
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    
    mesh
}

fn spawn_tile_meshes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    plane_chunk: &PlaneChunk,
    chunk_transform: Transform,
) {
    assert_eq!(plane_chunk.width % plane_chunk.grid_size, 0, "PlaneChunk width must be divisible by grid_size");
    assert_eq!(plane_chunk.height % plane_chunk.grid_size, 0, "PlaneChunk height must be divisible by grid_size");

    let grid_size = plane_chunk.grid_size;
    let tile_width = plane_chunk.width / grid_size;
    let tile_height = plane_chunk.height / grid_size;
    
    let invisible_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    
    for local_z in 0..grid_size {
        for local_x in 0..grid_size {
            // Calculate local tile position within chunk
            let local_x_offset = (local_x * tile_width) as f32 - (plane_chunk.width as f32 / 2.0) + (tile_width as f32 / 2.0);
            let local_z_offset = (local_z * tile_height) as f32 - (plane_chunk.height as f32 / 2.0) + (tile_height as f32 / 2.0);
            
            // Calculate global tile coordinates
            let global_x = plane_chunk.x * grid_size + local_x;
            let global_z = plane_chunk.z * grid_size + local_z;
            
            // Combine chunk position with local tile offset
            let tile_position = chunk_transform.translation + Vec3::new(local_x_offset, 0.01, local_z_offset);
            
            commands.spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(tile_width as f32, tile_height as f32))),
                MeshMaterial3d(invisible_mat.clone()),
                Transform::from_translation(tile_position),
                Pickable::default(),
                Tile { 
                    x: global_x, 
                    z: global_z,
                    ..default()
                },
                Name::new(format!("Tile ({}, {}) in Chunk ({}, {})", global_x, global_z, plane_chunk.x, plane_chunk.z)),
            ));
        }
    }
}

pub fn handle_chunk_clicks(
    mut click_events: MessageReader<Pointer<Click>>,
    mut hover_in_events: MessageReader<Pointer<Over>>,
    mut hover_out_events: MessageReader<Pointer<Out>>,
    mut chunk_query: Query<&mut Tile>,
) {

    for event in hover_in_events.read() {
        if let Ok(mut chunk) = chunk_query.get_mut(event.entity) {
            debug!("Hovered chunk: ({}, {})", chunk.x, chunk.z);
            chunk.selected = false;
            chunk.hovered = true;
        }
    }

    for event in hover_out_events.read() {
        if let Ok(mut chunk) = chunk_query.get_mut(event.entity) {
            debug!("Unhovered chunk: ({}, {})", chunk.x, chunk.z);
            chunk.hovered = false;
            chunk.selected = false;
        }
    }

    for event in click_events.read() {
        if let Ok(mut chunk) = chunk_query.get_mut(event.entity) {
            debug!("Clicked chunk: ({}, {})", chunk.x, chunk.z);
            chunk.selected = true;
            chunk.hovered = false;
        }
    }
}

pub fn draw_tiles_borders(
    mut gizmos: Gizmos,
    tile_query: Query<(&Transform, &Tile)>,
    test_plane_query: Query<&PlaneChunk>,
) {

    for test_plane in test_plane_query.iter() {

        let tile_width = test_plane.width / test_plane.grid_size;
        let tile_height = test_plane.height / test_plane.grid_size;

        for (transform, tile) in tile_query.iter() {
                
            draw_tile_borders(transform, &mut gizmos, tile_width, tile_height, 0.0, Color::BLACK);
        
            let color = match (tile.selected, tile.hovered) {
                (true, _) => tile.selected_color,      // Selected takes priority
                (false, true) => tile.hovered_color,   // Hovered but not selected
                (false, false) => tile.idle_color,     // Neither
            };

            // Draw filled rectangle inside borders
            draw_tile_borders(transform, &mut gizmos, tile_width, tile_height, -0.3, color);
        }
    }
}

fn draw_tile_borders(transform: &Transform, gizmos: &mut Gizmos, tile_width: i32, tile_height: i32, margin: f32, color: Color) {

    let half_width = (tile_width as f32 / 2.0) + margin;
    let half_height = (tile_height as f32 / 2.0) + margin;
    let center = transform.translation;

    // Draw rectangle borders
    gizmos.line(
        center + Vec3::new(-half_width, 0.0, -half_height),
        center + Vec3::new(half_width, 0.0, -half_height),
        color,
    );
    gizmos.line(
        center + Vec3::new(half_width, 0.0, -half_height),
        center + Vec3::new(half_width, 0.0, half_height),
        color,
    );
    gizmos.line(
        center + Vec3::new(half_width, 0.0, half_height),
        center + Vec3::new(-half_width, 0.0, half_height),
        color,
    );
    gizmos.line(
        center + Vec3::new(-half_width, 0.0, half_height),
        center + Vec3::new(-half_width, 0.0, -half_height),
        color,
    );
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