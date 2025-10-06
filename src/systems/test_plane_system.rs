use bevy::{asset::RenderAssetUsages, mesh::{Indices, PrimitiveTopology}, picking::hover, prelude::*, render::mesh, transform::commands};
use crate::components::{TestPlane, Tile};

pub fn init_test_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let test_plane = TestPlane::default();
    let mesh = create_test_plane_mesh(&test_plane);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: test_plane.color,
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        test_plane,
        Name::new("Test Plane"),
    ));
    info!("Test plane initialized");

    spawn_tile_meshes(&mut commands, meshes, materials, &test_plane.clone());
    info!("Tile meshes spawned");
}

fn create_test_plane_mesh(test_plane: &TestPlane) -> Mesh {
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
            uvs.push([u * 10.0, v * 10.0]); // Scale UVs for better texture mapping
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    test_plane: &TestPlane,
) {
    assert_eq!(test_plane.width % test_plane.grid_size, 0, "TestPlane width must be divisible by grid_size");
    assert_eq!(test_plane.height % test_plane.grid_size, 0, "TestPlane height must be divisible by grid_size");

    let grid_size = test_plane.grid_size;
    let tile_width = test_plane.width / grid_size;
    let tile_height = test_plane.height / grid_size;
    
    let invisible_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    
    for z in 0..grid_size {
        for x in 0..grid_size {
            
            // Calculate tile position
            let x_offset = (x * tile_width) as f32 - (test_plane.width as f32 / 2.0) + (tile_width as f32 / 2.0);
            let z_offset = (z * tile_height) as f32 - (test_plane.height as f32 / 2.0) + (tile_height as f32 / 2.0);
        
            commands.spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(tile_width as f32, tile_height as f32))),
                MeshMaterial3d(invisible_mat.clone()),
                Transform::from_xyz(x_offset, 0.01, z_offset), // Slightly above
                Pickable::default(),
                Tile { x, z, ..default() },
                Name::new(format!("Chunk ({}, {})", x, z)),
            ));

            info!("Spawned tile at ({}, {})", x, z);
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
            info!("Hovered chunk: ({}, {})", chunk.x, chunk.z);
            chunk.selected = false;
            chunk.hovered = true;
        }
    }

    for event in hover_out_events.read() {
        if let Ok(mut chunk) = chunk_query.get_mut(event.entity) {
            info!("Unhovered chunk: ({}, {})", chunk.x, chunk.z);
            chunk.hovered = false;
            chunk.selected = false;
        }
    }

    for event in click_events.read() {
        if let Ok(mut chunk) = chunk_query.get_mut(event.entity) {
            info!("Clicked chunk: ({}, {})", chunk.x, chunk.z);
            chunk.selected = true;
            chunk.hovered = false;
        }
    }
}

pub fn draw_tiles_borders(
    mut gizmos: Gizmos,
    tile_query: Query<(&Transform, &Tile)>,
    test_plane_query: Query<&TestPlane>,
) {

    let test_plane = test_plane_query.single().unwrap();
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
