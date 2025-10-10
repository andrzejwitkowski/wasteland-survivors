use bevy::prelude::*;
use crate::{components::TileSelectedEvent, systems::{player_movement_system::tile_selected_event_handle, tile_selection_system::handle_tile_selection}};

pub struct TileSelectionPlugin;

impl Plugin for TileSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TileSelectedEvent>()
            .add_systems(
                Update,
                handle_tile_selection.before(tile_selected_event_handle)
            );
    }
}