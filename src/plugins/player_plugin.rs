use bevy::prelude::*;

use crate::{components::PlayerMoveRequestEvent, systems::{draw_player_system::{draw_player, init_player}, player_movement_system::{execute_player_movement, plan_player_movement}}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_player)
            // draw_player adter startup init_player
            .add_systems(Update, draw_player)
            .add_message::<PlayerMoveRequestEvent>()
            .add_systems(Update, plan_player_movement)
            .add_systems(Update, execute_player_movement)
            ;
    }
}