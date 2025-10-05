use bevy::{asset::RenderAssetUsages, mesh::{Indices, PrimitiveTopology}, prelude::*, render::mesh};
use crate::components::TestPlane;

pub fn init_test_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let test_plane = TestPlane::default();
    // let mesh = create_test_plane_mesh(&test_plane);

    let mesh = Plane3d::default().mesh().size(test_plane.width as f32, test_plane.height as f32);

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
            let x_pos = u - 0.5;
            let z_pos = v - 0.5;
            
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