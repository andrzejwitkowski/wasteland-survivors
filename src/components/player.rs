use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub debug_gizmo: Option<DebugGizmo>,
    pub tile_entity: Option<Entity>,

    pub target_transform: Option<Transform>, // Current movement target
    pub path: VecDeque<Entity>,
    pub segment_start: Vec3, 
    pub translation_progress: f32, // 0.0 to 1.0
}

#[derive(Component)]
pub struct PlayerModel;

#[derive(Message)]
pub struct PlayerMoveRequestEvent {
    pub source_tile_entity: Entity,
    pub target_tile_entity: Entity,
}

// Optional: Component for debugging with gizmos
#[derive(Component)]
pub struct DebugGizmo {
    pub color: Color,
    pub size: f32,
}

impl Default for DebugGizmo {
    fn default() -> Self {
        Self {
            color: Color::srgb(1.0, 0.0, 0.0), // Red color
            size: 1.0,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 7.0,
            debug_gizmo: None,
            tile_entity: None,

            target_transform: None,
            path: VecDeque::new(),
            segment_start: Vec3::ZERO,
            translation_progress: 0.0,
        }
    }
}

impl Player {
    pub fn with_debug_gizmo(mut self, color: Color, size: f32) -> Self {
        self.debug_gizmo = Some(DebugGizmo { color, size });
        self
    }
}