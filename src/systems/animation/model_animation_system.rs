use crate::components::player::player::Player;
use crate::components::{AnimationGraphInitialized, ModelAnimationGraph, PlayAnimation};
use bevy::animation::AnimationPlayer;
use bevy::math::Vec3;
use bevy::prelude::{
    info, AnimationGraph, AnimationGraphHandle, AssetServer, Assets, Children, Commands,
    Entity, GltfAssetLabel, MessageReader, On, Query, Res, ResMut, Transform, Without,
};
use bevy::scene::{SceneInstanceReady, SceneRoot};
use std::collections::HashMap;

const PLAYER_MODEL_SCENE: &str = "models/dummy/dummy.glb#Scene0";
const PLAYER_MODEL: &str = "models/dummy/dummy.glb";
const PLAYER_SCALE: f32 = 5.0;
const WALK: &str = "walk";
const RUN: &str = "run";

pub fn init_animation_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    player_query: Query<(Entity, &Player), Without<ModelAnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();
    let root = graph.root;
    let blend = graph.add_blend(1.0, root);

    let scene_root = asset_server.load(PLAYER_MODEL_SCENE);

    let walk = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(PLAYER_MODEL)),
        0.0,
        blend,
    );

    let run = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(PLAYER_MODEL)),
        0.0,
        blend,
    );

    let graph_handle = graphs.add(graph);

    if let Some((player_entity, _)) = player_query.single().ok() {
        info!("Inserting ModelAnimationGraph component to player entity");

        commands
            .entity(player_entity)
            .insert((
                SceneRoot(scene_root.clone()),
                Transform::from_scale(Vec3::splat(PLAYER_SCALE)),
                ModelAnimationGraph {
                    graph: graph_handle,
                    animations: HashMap::from([(WALK.to_string(), walk), (RUN.to_string(), run)]),
                },
            ))
            .observe(on_animation_graph_loaded);
    }
}

pub fn on_animation_graph_loaded(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    model_animation_graph: Query<&ModelAnimationGraph>,
    initialized: Query<&AnimationGraphInitialized>,
) {
    if initialized.contains(scene_ready.entity) {
        return; // Already initialized, skip
    }

    info!("Animation graph loaded");
    if let Ok(model) = model_animation_graph.get(scene_ready.entity) {
        for child in children.iter_descendants(scene_ready.entity) {
            commands.entity(child).insert(AnimationGraphHandle(model.graph.clone()));
        }
        commands.entity(scene_ready.entity).insert(AnimationGraphInitialized);
    }
}

//TODO finish me
pub fn on_play_animation(
    mut events: MessageReader<PlayAnimation>,
    mut animation_players: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
    model_animation_graph: Query<&ModelAnimationGraph>,
) {
    for event in events.read() {
        if let Ok(model_graph) = model_animation_graph.get(event.model_animation_graph) {
            if let Some(&animation_id) = model_graph.animations.get(&event.animation_name) {
                for child in children.iter_descendants(event.model_animation_graph) {
                    if let Ok(mut player) = animation_players.get_mut(child) {
                        player.stop_all();
                        player.start(animation_id);
                    }
                }
            }
        }
    }
}
