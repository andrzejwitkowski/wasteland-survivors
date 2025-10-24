use bevy::prelude::{AnimationGraph, AnimationNodeIndex, Component, Entity, Handle, Message, Scene, Transform};
use std::collections::HashMap;
use bevy::scene::SceneRoot;

#[derive(Component)]
pub struct ModelAnimationGraph {
    pub graph: Handle<AnimationGraph>,
    pub animations: HashMap<String, AnimationNodeIndex>,
}

#[derive(Component)]
pub struct ModelSceneRoot {
    pub scene: SceneRoot,
    pub scene_transform: Transform
}

#[derive(Component)]
pub struct AnimationGraphInitialized;

#[derive(Message)]
pub struct PlayAnimation {
    pub animation_name: String,
    pub model_animation_graph: Entity,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum MovementState {
    Idle,
    Walking,
    Running,
}

#[derive(Component)]
pub struct InitialAnimationPlayed;

