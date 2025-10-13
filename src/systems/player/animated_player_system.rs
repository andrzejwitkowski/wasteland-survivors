use bevy::{platform::collections::HashMap, prelude::*};
use crate::components::{animated_model::{AnimationClip, AnimatedModel, PlayAnimationMessage}, player::player::Player};

const PLAYER_MODEL: &str = "models/dummy/dummy.glb#Scene0";
const PLAYER_SCALE: f32 = 5.0;

pub fn init_animated_player_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    player_query: Query<Entity, (With<Player>, Without<AnimatedModel>)>,
) {
    if let Ok(player) = player_query.single() {
        let model_handle = asset_server.load(PLAYER_MODEL);
        
        // Create animation clips
        let walk_clip = AnimationClip {
            index: AnimationNodeIndex::new(0),
            name: "walk".to_string(),
        };
        
        let run_clip = AnimationClip {
            index: AnimationNodeIndex::new(1),
            name: "run".to_string(),
        };

        // Create animation graph
        let (graph, _) = AnimationGraph::from_clips([
            asset_server.load("models/dummy/dummy.glb#Animation0"), // walk
            asset_server.load("models/dummy/dummy.glb#Animation1"), // run
        ]);
        
        let graph_handle = animation_graphs.add(graph);

        let mut animations = HashMap::new();
        animations.insert("walk".to_string(), walk_clip);
        animations.insert("run".to_string(), run_clip);

        commands.entity(player)
            .insert(Transform::from_scale(Vec3::splat(PLAYER_SCALE)))
            .insert(AnimatedModel {
                model_handle,
                animation_graph: graph_handle,
                animations,
            });

        info!("Player model initialized");
    }
}

pub fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    mut animation_events: MessageWriter<PlayAnimationMessage>,
) {
    if let Ok(player) = player_query.single() {
        if keyboard.just_pressed(KeyCode::Digit1) {
            animation_events.write(PlayAnimationMessage {
                entity: player,
                animation_name: "walk".to_string(),
            });
        }
        
        if keyboard.just_pressed(KeyCode::Digit2) {
            animation_events.write(PlayAnimationMessage {
                entity: player,
                animation_name: "run".to_string(),
            });
        }
    }
}