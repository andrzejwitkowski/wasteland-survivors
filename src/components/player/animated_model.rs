use bevy::{platform::collections::HashMap, prelude::*};

/// Generic component for any entity with animations
#[derive(Component)]
pub struct AnimatedModel {
    pub model_handle: Handle<Scene>,
    pub animation_graph: Handle<AnimationGraph>,
    pub animations: HashMap<String, AnimationClip>,
}

/// Individual animation clip reference
#[derive(Clone, Debug)]
pub struct AnimationClip {
    pub index: AnimationNodeIndex,
    pub name: String,
}

/// Animation state component
#[derive(Component)]
pub struct AnimationState {
    pub current_animation: Option<String>,
    pub is_playing: bool,
}

/// Event to play animations
#[derive(Message)]
pub struct PlayAnimationMessage {
    pub entity: Entity,
    pub animation_name: String,
}