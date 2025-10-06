use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub location: Vec3,
    pub rotation: Quat,
    pub debug_gizmo: Option<DebugGizmo>,
}

#[derive(Component)]
pub struct PlayerModel;

#[derive(Message)]
pub struct PlayerMoveRequestEvent {
    pub target_position: Vec3,
}

/// Marker component for player movement state
#[derive(Component)]
pub struct MovingToTarget {
    pub target_position: Vec3,
    pub speed: f32,
}

// Optional: Component for debugging with gizmos
#[derive(Component)]
pub struct DebugGizmo {
    pub color: Color,
    pub size: f32,
}

impl Default for DebugGizmo {
    fn default() -> Self {
        Self {
            color: Color::srgb(1.0, 0.0, 0.0), // Red color
            size: 1.0,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 10.0,
            location: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            debug_gizmo: None,
        }
    }
}

impl Player {
    pub fn with_debug_gizmo(mut self, color: Color, size: f32) -> Self {
        self.debug_gizmo = Some(DebugGizmo { color, size });
        self
    }
}