use bevy::prelude::*;
use crate::components::{player::player::Player, Tile, TileSelectedEvent};

/// Handles tile clicks and emits selection events
pub fn handle_tile_selection(
    mut click_events: MessageReader<Pointer<Click>>,
    tile_query: Query<&Tile>,
    player_query: Query<&Player>,
    mut tile_selected_events: MessageWriter<TileSelectedEvent>,
) {
    if player_query.is_empty() {
        // No player present, ignore tile selections
        return;
    }
    let player = player_query.single().unwrap();
    for event in click_events.read() {
        if let Ok(tile) = tile_query.get(event.entity) {
            info!("Tile clicked at ({}, {}), sending event", tile.x, tile.z);
            if let Some(player_tile_entity) = player.tile_entity {
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
                info!("Set player's current tile to {:?}", player.tile_entity);
            }
        }
    }
}