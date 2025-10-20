use bevy::prelude::{info, Changed, Entity, MessageWriter, Query, With};
use crate::components::{MovementState, PlayAnimation};
use crate::components::player::player::Player;
use crate::systems::animation::{IDLE, RUN, WALK};

pub fn movement_state_to_animation(
    mut play_animation_writer: MessageWriter<PlayAnimation>,
    state_query: Query<(Entity, &MovementState), (Changed<MovementState>, With<Player>)>
) {
    for (entity, movement_state) in state_query.iter() {
        let animation_name = match movement_state {
            MovementState::Idle => IDLE,
            MovementState::Walking => WALK,
            MovementState::Running => RUN,
        };

        play_animation_writer.write(PlayAnimation {
            animation_name: animation_name.to_string(),
            model_animation_graph: entity,
        });
    }
}