use bevy::prelude::*;

use crate::{components::PlayerMoveRequestEvent, systems::{draw_player_system::{draw_player, init_player}, player_movement_system::{player_movement_request_handler, tile_selected_event_handle, update_player_movement}}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_player)
            .add_systems(Update, draw_player)
            .add_message::<PlayerMoveRequestEvent>()
            .add_systems(
                Update,
                (
                    tile_selected_event_handle,
                    player_movement_request_handler.after(tile_selected_event_handle),
                    update_player_movement.after(player_movement_request_handler),
                )
            );
    }
}