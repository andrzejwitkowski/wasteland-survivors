use bevy::{prelude::*, scene::InstanceId};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub tile_entity: Option<Entity>,
}

#[derive(Component, Clone)]
pub struct PlayerAnimation {
    pub index: AnimationNodeIndex,
}

#[derive(Component)]
pub struct PlayerModel {
    pub model: Handle<Scene>,
    pub graph_handle: Handle<AnimationGraph>,
    pub walk_clip: Option<PlayerAnimation>,
    pub run_clip: Option<PlayerAnimation>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 7.0,
            tile_entity: None,
        }
    }
}