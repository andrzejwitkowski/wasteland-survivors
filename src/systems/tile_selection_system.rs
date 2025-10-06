use bevy::prelude::*;
use crate::components::{Tile, TileSelectedEvent};

/// Handles tile clicks and emits selection events
pub fn handle_tile_selection(
    mut click_events: MessageReader<Pointer<Click>>,
    tile_query: Query<&Tile>,
    mut tile_selected_events: MessageWriter<TileSelectedEvent>,
) {
    for event in click_events.read() {
        if let Ok(tile) = tile_query.get(event.entity) {
            info!("Tile clicked at ({}, {}), sending event", tile.x, tile.z);
            tile_selected_events.write(TileSelectedEvent {
                tile_entity: event.entity,
            });
        }
    }
}