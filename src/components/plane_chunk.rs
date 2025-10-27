use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct PlaneChunk {
    pub x: i32,
    pub z: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub grid_size: i32,
}

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub z: i32,
    pub walkable: bool,
    pub selected: bool,
    pub hovered: bool,
    pub idle_color: Color,
    pub selected_color: Color,
    pub hovered_color: Color,
    pub neighbor_entities: [Option<Entity>; 8], // Neighbor entities (N, NE, E, SE, S, SW, W, NW)
}

#[derive(Component)]
pub struct TilePosition {
    pub tile: Option<Entity>
}

#[derive(Resource, Default)]
pub struct TileRegistry {
    pub tiles_by_coord: HashMap<(i32, i32), Entity>,
}

#[derive(Message)]
pub struct TileSelectedEvent {
    pub source_tile_entity: Entity,
    pub target_tile_entity: Entity,
}

impl Default for PlaneChunk {
    fn default() -> Self {
        Self { x: 0, z: 0, width: 30, height: 30, color: Color::srgb(0.0, 1.0, 0.0), grid_size: 10 }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            x: 0,
            z: 0,
            walkable: true,
            selected: false,
            hovered: false,
            idle_color: Color::srgb(0.0, 0.0, 0.0),
            selected_color: Color::srgb(1.0, 0.0, 0.0),
            hovered_color: Color::srgb(0.0, 1.0, 0.0),
            neighbor_entities: [None; 8],
        }
    }
}

impl TilePosition {
    pub fn for_entity(entity: Entity) -> Self {
        Self { tile: Some(entity) }
    }
}
