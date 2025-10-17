use crate::components::player::player::Player;
use bevy::prelude::*;

pub fn init_player(mut commands: Commands) {
    let player_entity = (Transform::from_xyz(0.0, 0.0, 0.0), Player::default());

    commands.spawn(player_entity).insert(Name::new("Player"));
    info!("Player initialized");
}
