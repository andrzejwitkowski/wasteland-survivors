use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
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

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 7.0,
            tile_entity: None,

            target_transform: None,
            path: VecDeque::new(),
            segment_start: Vec3::ZERO,
            translation_progress: 0.0,
        }
    }
}