use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component, Clone)]
pub struct PlayerAnimation {
    pub index: AnimationNodeIndex,
}

#[derive(Message)]
pub struct PlayerStartupTileSelectedEvent {
    pub tile_entity: Entity,
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
        Self { speed: 10.0 }
    }
}
