use bevy::prelude::*;

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