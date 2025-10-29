use bevy::prelude::*;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingState, LoadingStateAppExt};
use PlayerLoadingState::Loading;
use crate::systems::animation::{check_animations_loaded, init_animation_system, movement_state_to_animation, on_play_animation, play_animation_system, start_initial_animation, PendingAnimations, PlayerAssets, PlayerLoadingState};
use crate::systems::movement::movement_system::{
    init_player_movement, movement_request_handler, tile_selected_event_handle,
    update_player_movement,
};

use crate::components::PlayAnimation;
use crate::player::player_system::{init_player, init_player_startup_tile};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Input,
    Movement,
    Update,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerLoadingState>()
            .add_loading_state(
                LoadingState::new(Loading)
                    .continue_to_state(PlayerLoadingState::Ready)
                    .load_collection::<PlayerAssets>(),
            )
            .add_systems(OnEnter(PlayerLoadingState::Ready), init_animation_system.after(init_player_movement))
            .add_systems(OnEnter(PlayerLoadingState::Ready), play_animation_system.after(init_animation_system))
            .add_systems(OnEnter(PlayerLoadingState::Ready), init_player_startup_tile.after(init_animation_system))
        ;

        app
            .add_message::<PlayAnimation>()
            .add_systems(Startup, init_player)
            .add_systems(Startup, init_player_movement.after(init_player))
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
                    check_animations_loaded
                        .run_if(any_with_component::<PendingAnimations>),
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
