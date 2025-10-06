use bevy::prelude::*;
use crate::components::{MovingToTarget, Player, PlayerMoveRequestEvent, Tile, TileSelectedEvent};

pub fn plan_player_movement(
    mut tile_selected_events: MessageReader<TileSelectedEvent>,
    player_query: Query<&Player>,
    tile_query: Query<&Transform, With<Tile>>,
    mut player_move_events: MessageWriter<PlayerMoveRequestEvent>,
) {
    if let Ok(_) = player_query.single() {
        for event in tile_selected_events.read() {
            // Here we would normally get the tile's position from its Transform
            // For simplicity, let's assume each tile is 1 unit apart and we can derive position from its entity ID
            if let Ok(tile_transform) = tile_query.get(event.tile_entity) {
                let tile_position = tile_transform.translation;

                info!("Planning movement to tile entity: {:?} at position: {:?}", event.tile_entity, tile_position);
                player_move_events.write(PlayerMoveRequestEvent {
                    target_position: tile_position,
                });
            }
        }
    } else {
        warn!("No player found to move");
    }
}

pub fn execute_player_movement(
    mut player_move_events: MessageReader<PlayerMoveRequestEvent>,
    mut player_query: Query<(&mut Transform, &Player), Without<MovingToTarget>>,
    mut commands: Commands,
) {
    for event in player_move_events.read() {
        if let Ok((mut transform, player)) = player_query.single_mut() {
            info!("Executing movement to position: {:?}", event.target_position);

            // Here we would normally implement pathfinding logic
            // For simplicity, let's just move the player directly to the target position
            // TODO - Add pathfinding logic here
            transform.translation = event.target_position;

            // Optionally, add a MovingToTarget component if you want to animate movement over time
            // commands.entity(player).insert(MovingToTarget {
            //     target_position: event.target_position,
            //     speed: player.move_speed,
            // });
        } else {
            warn!("No player found to execute movement");
        }
    }
}