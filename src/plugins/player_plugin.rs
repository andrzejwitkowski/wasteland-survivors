use bevy::prelude::*;

use crate::components::movements::movement::MoveRequestEvent;
use crate::systems::animation::{init_animation_system, movement_state_to_animation, on_play_animation, start_initial_animation};
use crate::systems::movement::movement_system::{
    init_player_movement, movement_request_handler, tile_selected_event_handle,
    update_player_movement,
};
use crate::{
    components::animated_model::PlayAnimationMessage, systems::player::player_system::init_player,
};
use crate::components::PlayAnimation;
use crate::components::player::player::PlayerStartupTileSelectedEvent;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Input,
    Movement,
    Update,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MoveRequestEvent>()
            .add_message::<PlayAnimationMessage>()
            .add_message::<PlayAnimation>()
            .add_message::<PlayerStartupTileSelectedEvent>()
            .add_systems(Startup, init_player)
            .add_systems(Startup, init_player_movement.after(init_player))
            .add_systems(Startup, init_animation_system.after(init_player_movement))
            .configure_sets(
                Update,
                (
                    crate::plugins::tile_selection_plugin::InputSet,
                    PlayerSystemSet::Input.after(crate::plugins::tile_selection_plugin::InputSet),
                    PlayerSystemSet::Movement.after(PlayerSystemSet::Input),
                    PlayerSystemSet::Update.after(PlayerSystemSet::Movement),
                ),
            )
            .add_systems(
                Update,
                (
                    tile_selected_event_handle.in_set(PlayerSystemSet::Input),
                    movement_request_handler.in_set(PlayerSystemSet::Movement),
                    update_player_movement.in_set(PlayerSystemSet::Update),
                    movement_state_to_animation.in_set(PlayerSystemSet::Update),
                    start_initial_animation.in_set(PlayerSystemSet::Update),
                    on_play_animation.in_set(PlayerSystemSet::Update),
                ),
            );
    }
}
