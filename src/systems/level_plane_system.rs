use bevy::prelude::*;
use crate::{systems::plane_chunk_system::spawn_single_chunk_grid};

pub struct LevelPlaneConfig {
    pub num_cols: i32,
    pub num_rows: i32,
    pub chunk_width: i32,
    pub chunk_height: i32,
    pub color: Color,
    pub grid_size: i32,
}

impl Default for LevelPlaneConfig {
    fn default() -> Self {
        Self {
            num_cols: 3,
            num_rows: 3,
            chunk_width: 30,
            chunk_height: 30,
            color: Color::srgb(0.0, 1.0, 0.0),
            grid_size: 10,
        }
    }
}

pub fn spawn_default_chunk_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let level_plane_config = LevelPlaneConfig::default();

    spawn_chunk_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        level_plane_config.num_cols,
        level_plane_config.num_rows,
        level_plane_config.chunk_width,
        level_plane_config.chunk_height,
        level_plane_config.grid_size,
    );
}

/// Spawns a grid of plane chunks arranged in columns and rows
/// 
/// # Arguments
/// * `num_cols` - Number of columns in the grid
/// * `num_rows` - Number of rows in the grid
/// * `chunk_width` - Width of each individual chunk
/// * `chunk_height` - Height of each individual chunk
/// * `grid_size` - Number of tiles per chunk side
pub fn spawn_chunk_grid(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    num_cols: i32,
    num_rows: i32,
    chunk_width: i32,
    chunk_height: i32,
    grid_size: i32,
) {
    info!("Spawning chunk grid: {}x{} chunks", num_cols, num_rows);
    
        info!("Spawning chunk grid: {}x{} chunks", num_cols, num_rows);
    
    for row in 0..num_rows {
        for col in 0..num_cols {
            // Calculate chunk position based on column and row
            spawn_single_chunk_grid(
                &mut commands,
                &mut meshes,
                &mut materials,
                col,
                row,
                num_cols,
                num_rows,
                chunk_width,
                chunk_height,
                grid_size,
            )
        }
    }
}