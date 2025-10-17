use bevy::prelude::*;
use std::collections::VecDeque;

pub enum MovementType {
    ASTAR,
    SHORTEST,
}
#[derive(Component)]
pub struct Movement {
    pub target_transform: Option<Transform>, // Current movement target
    pub path: VecDeque<Entity>,
    pub segment_start: Vec3,
    pub translation_progress: f32,
    pub segment_distance: f32,
}

#[derive(Message)]
pub struct MoveRequestEvent {
    pub source_tile_entity: Entity,
    pub target_tile_entity: Entity,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            target_transform: None,
            path: VecDeque::new(),
            segment_start: Vec3::ZERO,
            translation_progress: 0.0,
            segment_distance: 0.0,
        }
    }
}
