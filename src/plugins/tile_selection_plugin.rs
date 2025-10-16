use bevy::prelude::*;
use crate::{components::TileSelectedEvent, systems::{tile_selection_system::handle_tile_selection}};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InputSet;
pub struct TileSelectionPlugin;

impl Plugin for TileSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TileSelectedEvent>()
            .configure_sets(
                Update,
                (
                    InputSet,
                )
            )
            .add_systems(
                Update,
                handle_tile_selection.in_set(InputSet)
            );
    }
}