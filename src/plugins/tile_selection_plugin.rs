use bevy::prelude::*;
use crate::{components::TileSelectedEvent, systems::tile_selection_system::handle_tile_selection};

pub struct TileSelectionPlugin;

impl Plugin for TileSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TileSelectedEvent>()
            .add_systems(Update, handle_tile_selection);
    }
}