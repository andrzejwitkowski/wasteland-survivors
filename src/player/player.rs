use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Message)]
pub struct PlayerStartupTileSelectedEvent {
    pub tile_entity: Entity,
}
