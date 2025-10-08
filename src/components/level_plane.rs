use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct LevelPlane {
    pub chunk_num_x: i32,
    pub chunk_num_y: i32,
    pub chunk_width: i32,
    pub chunk_height: i32,
    pub color: Color,
    pub grid_size: i32,
}

impl Default for LevelPlane {
    fn default() -> Self {
        Self {
            chunk_num_x: 3,
            chunk_num_y: 3,
            chunk_width: 30,
            chunk_height: 30,
            color: Color::srgb(0.0, 1.0, 0.0),
            grid_size: 10,
        }
    }
}