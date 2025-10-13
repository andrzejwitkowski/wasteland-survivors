use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub tile_entity: Option<Entity>,
}

#[derive(Component)]
pub struct PlayerModel;

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 7.0,
            tile_entity: None,
        }
    }
}