use crate::components::player::player::Player;
use bevy::prelude::*;
use crate::components::{PlaneChunk, TilePosition, TileRegistry};

pub fn init_player(mut commands: Commands) {
    let player_entity = (Player::default());
    commands.spawn(player_entity).insert(Name::new("Player"));
    info!("Player initialized");
}

pub fn init_player_startup_tile(
    mut commands: Commands,
    mut player_query: Query<(Entity), (With<Player>, Without<Transform>)>,
    tile_registry: Res<TileRegistry>,
    grid_query: Query<(&Transform, &PlaneChunk), Without<Player>>,
) {
    if let Some(player_entity) = player_query.single_mut().ok() {
        if let Some(middle) = crate::systems::plane_chunk_system::get_middle(&tile_registry.tiles_by_coord) {
            commands.entity(player_entity).insert(
                TilePosition {
                    tile: Some(middle.1.clone())
                }
            );
            if let Some(world_pos) = crate::systems::plane_chunk_system::calculate_tile_world_position(*middle.0, &grid_query) {
                commands.entity(player_entity).insert(Transform::from_translation(world_pos));
                info!("Player positioned at middle tile: {:?}", world_pos);
            }
        }
    }
}
