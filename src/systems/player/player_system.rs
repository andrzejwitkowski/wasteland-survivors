use crate::components::movements::movement::MovementSpeed;
use crate::components::player::player::{Player, PlayerStartupTileSelectedEvent};
use crate::components::{PlaneChunk, TilePosition, TileRegistry};
use crate::shared::CharacterType;
use bevy::prelude::*;

pub fn init_player(mut commands: Commands) {
    commands.spawn(Player {}).insert((
        MovementSpeed::default(),
        CharacterType::Player,
        Name::new("Player"),
    ));
    info!("Player initialized");
}

pub fn init_player_startup_tile(
    mut commands: Commands,
    mut player_query: Query<(Entity), (With<Player>, Without<Transform>)>,
    mut player_startup_tile_selected_events: MessageWriter<PlayerStartupTileSelectedEvent>,
    tile_registry: Res<TileRegistry>,
    grid_query: Query<(&Transform, &PlaneChunk), Without<Player>>,
) {
    if let Some(player_entity) = player_query.single_mut().ok() {
        if let Some(middle) =
            crate::systems::plane_chunk_system::get_middle(&tile_registry.tiles_by_coord)
        {
            commands.entity(player_entity).insert(TilePosition { tile: Some(middle.1.clone()) });
            if let Some(world_pos) =
                crate::systems::plane_chunk_system::calculate_tile_world_position(
                    *middle.0,
                    &grid_query,
                )
            {
                commands.entity(player_entity).insert(Transform::from_translation(world_pos));
                info!("Player positioned at middle tile: {:?}", world_pos);
            }

            player_startup_tile_selected_events
                .write(PlayerStartupTileSelectedEvent { tile_entity: middle.1.clone() });
            info!("Player startup tile selected");
        }
    }
}
