use bevy::prelude::*;

use crate::{components::PlayerMoveRequestEvent, systems::{draw_player_system::{draw_player, init_player}, player_movement_system::{move_along_path, player_movement_request_handler, tile_selected_event_handle}}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_player)
            // draw_player adter startup init_player
            .add_systems(Update, draw_player)
            .add_message::<PlayerMoveRequestEvent>()
            .add_systems(Update, tile_selected_event_handle)
            .add_systems(Update, player_movement_request_handler)
            .add_systems(Update, move_along_path)
            ;
    }
}