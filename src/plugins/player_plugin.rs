use bevy::prelude::*;

use crate::{components::player::movement::PlayerMoveRequestEvent, systems::player::{
        gizmo_system::{draw_player, set_player_gizmo_debug}, 
        movement_system::{init_player_movement, player_movement_request_handler, tile_selected_event_handle, update_player_movement}, player_system::init_player}};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Input,
    Movement,
    Update,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_player)
            .add_systems(Startup, init_player_movement.after(init_player))
            .add_systems(Update, set_player_gizmo_debug)
            .add_systems(Update, draw_player)
            .add_message::<PlayerMoveRequestEvent>()
            .configure_sets(
                Update,
                (
                    crate::plugins::tile_selection_plugin::InputSet,
                    PlayerSystemSet::Input.after(crate::plugins::tile_selection_plugin::InputSet),
                    PlayerSystemSet::Movement.after(PlayerSystemSet::Input),
                    PlayerSystemSet::Update.after(PlayerSystemSet::Movement),
                )
            )
            .add_systems(
                Update,
                (
                    tile_selected_event_handle.in_set(PlayerSystemSet::Input),
                    player_movement_request_handler.in_set(PlayerSystemSet::Movement),
                    update_player_movement.in_set(PlayerSystemSet::Update),
                )
            );
    }
}