use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Player;

#[derive(Message)]
pub struct PlayerStartupTileSelectedEvent {
    pub tile_entity: Entity,
}
