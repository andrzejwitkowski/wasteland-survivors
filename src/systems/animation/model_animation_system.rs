use crate::components::player::player::Player;
use crate::components::{AnimationGraphInitialized, ModelAnimationGraph, PlayAnimation};
use bevy::animation::AnimationPlayer;
use bevy::math::Vec3;
use bevy::prelude::{
    AnimationGraph, AnimationGraphHandle, AnimationTransitions, AssetServer, Assets, ChildOf,
    Children, Commands, Entity, GltfAssetLabel, MessageReader, MessageWriter, On, Query, Res,
    ResMut, Transform, With, Without, info,
};
use bevy::scene::{SceneInstanceReady, SceneRoot};
use std::collections::HashMap;
use std::time::Duration;

const PLAYER_MODEL_SCENE: &str = "models/dummy/dummy.glb#Scene0";

const PLAYER_MODEL_IDLE: &str = "models/dummy/dummy.glb#Animation0";

const PLAYER_MODEL_RUN: &str = "models/dummy/dummy.glb#Animation1";
const PLAYER_MODEL_WALK: &str = "models/dummy/dummy.glb#Animation2";

const PLAYER_MODEL: &str = "models/dummy/dummy.glb";
const PLAYER_SCALE: f32 = 5.0;

pub const IDLE: &str = "idle";
pub const WALK: &str = "walk";
pub const RUN: &str = "run";

const TRANSITION_DURATION: f32 = 0.3;

pub fn init_animation_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    player_query: Query<(Entity, &Player), Without<ModelAnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();
    let root = graph.root;

    let scene_root = asset_server.load(PLAYER_MODEL_SCENE);

    let walk = graph.add_clip(asset_server.load(PLAYER_MODEL_WALK), 1.0, root);

    let run = graph.add_clip(asset_server.load(PLAYER_MODEL_RUN), 1.0, root);

    let idle = graph.add_clip(asset_server.load(PLAYER_MODEL_IDLE), 1.0, root);

    let graph_handle = graphs.add(graph);

    if let Some((player_entity, _)) = player_query.single().ok() {
        info!("Inserting ModelAnimationGraph component to player entity");

        // Spawn the scene as a child entity to prevent Transform overwriting
        let scene_entity = commands
            .spawn((
                SceneRoot(scene_root.clone()),
                Transform::from_scale(Vec3::splat(PLAYER_SCALE)),
            ))
            .observe(on_animation_graph_loaded)
            .id();

        commands
            .entity(player_entity)
            .insert((
                ModelAnimationGraph {
                    graph: graph_handle,
                    animations: HashMap::from([
                        (WALK.to_string(), walk),
                        (RUN.to_string(), run),
                        (IDLE.to_string(), idle),
                    ]),
                },
                AnimationTransitions::new(),
            ))
            .add_child(scene_entity);
    }
}

fn on_animation_graph_loaded(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    model_animation_graph: Query<(Entity, &ModelAnimationGraph, &Children), With<Player>>,
    already_initialized: Query<&AnimationGraphInitialized>,
) {
    info!("Scene ready for entity: {:?}", scene_ready.entity);

    // Guard: Skip if already initialized (SceneInstanceReady can fire multiple times)
    if already_initialized.get(scene_ready.entity).is_ok() {
        info!("Animation graph already initialized, skipping duplicate scene ready event");
        return;
    }

    // Find the player entity that has this scene as a child
    for (player_entity, model, player_children) in model_animation_graph.iter() {
        // Check if scene_ready.entity is a child of this player
        if player_children.contains(&scene_ready.entity) {
            // Add AnimationGraphHandle to all descendants of the scene
            for child in children.iter_descendants(scene_ready.entity) {
                commands.entity(child).insert((
                    AnimationGraphHandle(model.graph.clone()),
                    AnimationTransitions::new(),
                ));
            }
            info!("Added AnimationGraphHandle to all children");

            // Mark the SCENE entity as initialized
            commands.entity(scene_ready.entity).insert(AnimationGraphInitialized);

            // Also mark the PLAYER entity to trigger initial animation
            commands.entity(player_entity).insert(AnimationGraphInitialized);
            break;
        }
    }
}

pub fn start_initial_animation(
    mut commands: Commands,
    mut play_animation_writer: MessageWriter<PlayAnimation>,
    query: Query<
        Entity,
        (
            With<Player>,
            With<AnimationGraphInitialized>,
            Without<crate::components::InitialAnimationPlayed>,
        ),
    >,
) {
    for entity in query.iter() {
        info!("Starting initial IDLE animation for player");
        play_animation_writer.write(PlayAnimation {
            animation_name: IDLE.to_string(),
            model_animation_graph: entity,
        });
        // Mark that we started the initial animation so this only happens once
        commands.entity(entity).insert(crate::components::InitialAnimationPlayed);
    }
}

pub fn on_play_animation(
    mut events: MessageReader<PlayAnimation>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    children: Query<&Children>,
    model_animation_graph: Query<&ModelAnimationGraph>,
) {
    for event in events.read() {
        if let Ok(model_graph) = model_animation_graph.get(event.model_animation_graph) {
            for child in children.iter_descendants(event.model_animation_graph) {
                if let Ok((mut player, mut transitions)) = animation_players.get_mut(child) {
                    // player.stop_all();
                    if let Some(&animation_id) = model_graph.animations.get(&event.animation_name) {
                        info!("playing animation: {}", event.animation_name);
                        transitions
                            .play(
                                &mut player,
                                animation_id,
                                Duration::from_secs_f32(TRANSITION_DURATION),
                            )
                            .repeat();
                    }
                }
            }
        }
    }
}
