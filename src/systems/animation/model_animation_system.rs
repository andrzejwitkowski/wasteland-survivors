use crate::components::player::player::Player;
use crate::components::{ModelAnimationGraph, PlayAnimation};
use bevy::animation::AnimationPlayer;
use bevy::math::Vec3;
use bevy::prelude::{
    AnimationGraph, AnimationGraphHandle, AnimationTransitions, AssetServer, Assets, ChildOf,
    Children, Commands, Component, Entity, GltfAssetLabel, MessageReader, MessageWriter, On, Query,
    Res, ResMut, Transform, With, Without, info,
};
use bevy::scene::{SceneInstanceReady, SceneRoot};
use std::collections::HashMap;
use std::time::Duration;

const PLAYER_MODEL_SCENE: &str = "models/dummy/dummy.glb#Scene0";

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

    let scene_root = asset_server.load(PLAYER_MODEL_SCENE); // → SceneInstanceReady RAZ!

    let idle = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(PLAYER_MODEL)),
        1.0,
        root,
    );
    let run = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(PLAYER_MODEL)),
        1.0,
        root,
    );
    let walk = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(PLAYER_MODEL)),
        1.0,
        root,
    );

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
    model_animation_graph: Query<(&ModelAnimationGraph, &Children), With<Player>>,
) {
    info!("Scene ready for entity: {:?}", scene_ready.entity);

    // Find the player entity that has this scene as a child
    for (model, player_children) in model_animation_graph.iter() {
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

            break;
        }
    }
}

#[derive(Component)]
pub struct AnimationGraphInitialized;

pub fn start_initial_animation(
    mut commands: Commands,
    children: Query<&Children>,
    players: Query<(Entity, &ModelAnimationGraph), With<Player>>,
    mut animation_players: Query<
        (Entity, &mut AnimationPlayer, &mut AnimationTransitions),
        (With<AnimationGraphHandle>, Without<AnimationGraphInitialized>),
    >,
) {
    for (player_entity, model_graph) in &players {
        // ✅ Klasyczna pętla for - działa z &mut
        for child in children.iter_descendants(player_entity) {
            if let Ok((entity, mut player, mut transitions)) = animation_players.get_mut(child) {
                play_idle_animation(model_graph, &mut player, &mut transitions);
                commands.entity(entity).insert(AnimationGraphInitialized);
                info!("Initialized IDLE animation for entity: {:?}", entity);
            }
        }
    }
}

#[inline]
fn play_idle_animation(
    model_graph: &ModelAnimationGraph,
    player: &mut AnimationPlayer,
    transitions: &mut AnimationTransitions,
) {
    if let Some(&idle_id) = model_graph.animations.get(IDLE) {
        transitions
            .play(player, idle_id, Duration::from_secs_f32(TRANSITION_DURATION))
            .repeat();
    }
}

pub fn on_play_animation(
    mut events: MessageReader<PlayAnimation>,
    children: Query<&Children>,
    model_animation_graph: Query<&ModelAnimationGraph>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    for event in events.read() {
        let Ok(model_graph) = model_animation_graph.get(event.model_animation_graph) else {
            continue;
        };

        info!("Playing animation: {}", event.animation_name);

        for child in children.iter_descendants(event.model_animation_graph) {
            if let Ok((mut player, mut transitions)) = animation_players.get_mut(child) {
                if let Some(&animation_id) = model_graph.animations.get(&event.animation_name) {
                    transitions
                        .play(&mut player, animation_id, Duration::from_secs_f32(TRANSITION_DURATION))
                        .repeat();
                    info!("Started animation: {}", event.animation_name);
                }
            }
        }
    }
}

