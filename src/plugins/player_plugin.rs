use bevy::prelude::*;

use crate::systems::draw_player_system::{draw_player, init_player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_player)
            // draw_player adter startup init_player
            .add_systems(Update, draw_player)
            ;
    }
}