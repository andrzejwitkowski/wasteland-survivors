use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct TestPlane {
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub grid_size: i32,
}

#[derive(Component)]
pub struct Tile{
    pub x: i32,
    pub z: i32,
    pub walkable: bool,
    pub selected: bool,
    pub hovered: bool,
    pub idle_color: Color,
    pub selected_color: Color,
    pub hovered_color: Color,
}

#[derive(Message)]
pub struct TileSelectedEvent {
    pub tile_entity: Entity
}

impl Default for TestPlane {
    fn default() -> Self {
        Self {
            width: 30,
            height: 30,
            color: Color::srgb(0.0, 1.0, 0.0),
            grid_size: 10,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            x: 0,
            z: 0,
            walkable: false,
            selected: false,
            hovered: false,
            idle_color: Color::srgb(0.0, 0.0, 0.0),
            selected_color: Color::srgb(1.0, 0.0, 0.0),
            hovered_color: Color::srgb(0.0, 1.0, 0.0),
        }
    }
}