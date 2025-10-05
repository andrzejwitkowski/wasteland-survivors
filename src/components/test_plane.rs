use bevy::prelude::*;

#[derive(Component)]
pub struct TestPlane {
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub grid_size: i32,
}

impl Default for TestPlane {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            color: Color::srgb(0.0, 1.0, 0.0),
            grid_size: 10,
        }
    }
}