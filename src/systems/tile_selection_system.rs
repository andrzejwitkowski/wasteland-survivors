use crate::components::{Tile, TileSelectedEvent, player::player::Player, TilePosition};
use bevy::prelude::*;

/// Handles tile clicks and emits selection events
pub fn handle_tile_selection(
    mut click_events: MessageReader<Pointer<Click>>,
    tile_query: Query<&Tile>,
    player_query: Query<Entity, (With<Player>, With<TilePosition>)>,
    tile_positions_query: Query<&TilePosition>,
    mut tile_selected_events: MessageWriter<TileSelectedEvent>,
) {
    if player_query.is_empty() {
        // No player present, ignore tile selections
        return;
    }
    let player_entity = player_query.single().unwrap();
    if let Some(player_tile_position) = tile_positions_query.get(player_entity).ok() {
        for event in click_events.read() {
            if let Ok(tile) = tile_query.get(event.entity) {
                info!("Tile clicked at ({}, {}), sending event", tile.x, tile.z);

                if let Some(player_tile_entity) = player_tile_position.tile {
                    if player_tile_entity == event.entity {
                        // Clicked on the tile where the player is located, ignore
                        info!("Clicked on player's current tile, ignoring");
                        continue;
                    }
                    // Emit event with both source and target tile entities
                    tile_selected_events.write(TileSelectedEvent {
                        source_tile_entity: player_tile_entity,
                        target_tile_entity: event.entity,
                    });
                } else {
                    // First time selecting a tile, set as player's current tile
                    tile_selected_events.write(TileSelectedEvent {
                        source_tile_entity: event.entity,
                        target_tile_entity: event.entity,
                    });
                }
            }
        }
    }
}
