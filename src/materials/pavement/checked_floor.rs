use bevy::prelude::*;

#[derive(Resource)]
pub struct CheckedFloorMaterials {
    pub material: Handle<StandardMaterial>,
}

const CHECKED_FLOOR_TEXTURE_PATH: &str =
    "textures/pavement/checked/checkered_pavement_tiles_diff_1k.png";
const CHECKED_FLOOR_TEXTURE_NORMAL_PATH: &str =
    "textures/pavement/checked/checkered_pavement_tiles_nor_gl_1k.png"; // Use OpenGL normal map
const CHECKED_FLOOR_TEXTURE_ARM_PATH: &str =
    "textures/pavement/checked/checkered_pavement_tiles_arm_1k.png";

impl FromWorld for CheckedFloorMaterials {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let base_texture = asset_server.load(CHECKED_FLOOR_TEXTURE_PATH);
        let normal_texture = asset_server.load(CHECKED_FLOOR_TEXTURE_NORMAL_PATH);
        let arm_texture = asset_server.load(CHECKED_FLOOR_TEXTURE_ARM_PATH);

        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(base_texture),
            base_color: Color::srgb(1.0, 1.0, 1.0), // White to preserve texture colors

            normal_map_texture: Some(normal_texture),

            // Use ARM texture for ambient occlusion and metallic
            occlusion_texture: Some(arm_texture.clone()),
            metallic_roughness_texture: Some(arm_texture),

            // Use separate roughness texture for more control
            perceptual_roughness: 0.8, // Base value, will be modulated by texture
            metallic: 0.0,             // Non-metallic surface

            ..Default::default()
        });

        CheckedFloorMaterials { material }
    }
}
