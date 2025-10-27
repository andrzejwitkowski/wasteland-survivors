use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

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
