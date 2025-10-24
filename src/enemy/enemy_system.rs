use crate::components::TileRegistryCreatedEvent;
use crate::enemy::enemy_components::{Enemy, EnemyGizmo};
use bevy::math::IVec2;
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

pub struct EnemyConfig {
    pub enemy_count: i32,
    pub enemy_player_min_distance: f32,
    pub enemy_enemy_min_distance: f32,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self { enemy_count: 10, enemy_player_min_distance: 10.0, enemy_enemy_min_distance: 10.0 }
    }
}

pub fn init_enemy(
    mut commands: Commands,
    mut tile_registry_created_events: MessageReader<TileRegistryCreatedEvent>,
) {
    let mut enemy_config = EnemyConfig::default();
    for event in tile_registry_created_events.read() {
        info!("TileRegistry created, initializing enemy system");
        // commands.spawn((
        //     Enemy::default(),
        //     EnemyGizmo::default())
        // );
    }
    info!("Enemy system initialized");
}

/// Znajduje losową wolną pozycję na planszy w określonym zakresie odległości
/// od gracza i z minimalną odległością od wrogów.
///
/// # Arguments
/// * `tiles_by_coord` - Mapa wszystkich kafelków na planszy
/// * `player_tile_pos` - Pozycja gracza
/// * `enemy_tile_positions` - Wektor pozycji wrogów
/// * `min_distance_from_player` - Minimalna odległość od gracza (w kafelkach)
/// * `max_distance_from_player` - Maksymalna odległość od gracza (w kafelkach)
/// * `min_distance_from_enemies` - Minimalna odległość od każdego wroga (w kafelkach)
///
/// # Returns
/// `Option<(IVec2, TileData)>` - Znaleziony kafelek lub None jeśli nie znaleziono
pub fn find_spawn_position(
    tiles_by_coord: &HashMap<IVec2, Entity>,
    player_tile_pos: IVec2,
    enemy_tile_positions: &[IVec2],
    min_distance_from_player: f32,
    max_distance_from_player: f32,
    min_distance_from_enemies: f32,
) -> Option<(IVec2, Entity)> {
    let mut rng = rand::rng();
    let player_vec = player_tile_pos.as_vec2();

    // Prefiltruj najpierw odległość od gracza (najbardziej ograniczający warunek)
    let valid_tiles: Vec<_> = tiles_by_coord
        .iter()
        .filter(|(coord, _)| {
            let coord_vec = coord.as_vec2();
            let distance_to_player = coord_vec.distance(player_vec);

            // Szybkie odrzucenie: sprawdź zakres od gracza
            if distance_to_player < min_distance_from_player
                || distance_to_player > max_distance_from_player
            {
                return false;
            }

            // Sprawdź odległość od każdego wroga
            enemy_tile_positions.iter().all(|enemy_pos| {
                let enemy_vec = enemy_pos.as_vec2();
                coord_vec.distance(enemy_vec) >= min_distance_from_enemies
            })
        })
        .collect();

    // Wybierz losowy kafelek z dostępnych
    if valid_tiles.is_empty() {
        None
    } else {
        let index = rng.random_range(0..valid_tiles.len());
        valid_tiles.get(index).map(|(coord, tile)| (**coord, *tile.clone()))
    }
}
