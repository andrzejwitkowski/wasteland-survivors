use crate::components::player::player::{Player, PlayerStartupTileSelectedEvent};
use crate::components::{MovementState, Tile, TilePosition, TileRegistry};
use crate::enemy;
use crate::enemy::enemy_components::{Enemy, EnemyGizmo, EnemyLastMovementTime, EnemySpawned};
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use crate::components::movements::movement::{Movement, MovementSpeed};
use crate::shared::CharacterType;

pub struct EnemyConfig {
    pub enemy_count: i32,
    pub enemy_player_min_distance: f32,
    pub enemy_player_max_distance: f32,
    pub enemy_enemy_min_distance: f32,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            enemy_count: 10,
            enemy_player_min_distance: 10.0,
            enemy_player_max_distance: 30.0,
            enemy_enemy_min_distance: 5.0,
        }
    }
}

pub fn init_enemy(
    mut commands: Commands,
    mut tile_registry_created_events: MessageReader<PlayerStartupTileSelectedEvent>,
    tile_registry: Res<TileRegistry>,
    tiles_query: Query<(Entity, &Tile, &Transform)>,
    enemy_spawned_query: Query<&EnemySpawned>,
) {
    if !enemy_spawned_query.is_empty() {
        return;
    }

    let enemy_config = EnemyConfig::default();
    for event in tile_registry_created_events.read() {
        info!("TileRegistry created, initializing enemy system");

        if let Ok((_, tile, _)) = tiles_query.get(event.tile_entity) {
            let mut enemies = Vec::new();
            for _ in 0..enemy_config.enemy_count {
                if let Some(enemy) = find_spawn_position(
                    &tile_registry.tiles_by_coord,
                    (tile.x, tile.z),
                    enemies.as_slice(),
                    enemy_config.enemy_player_min_distance,
                    enemy_config.enemy_player_max_distance,
                    enemy_config.enemy_enemy_min_distance,
                ) {
                    info!("Spawning enemy at {:?}", enemy);
                    enemies.push(enemy.0);

                    if let Ok((entity, _, transform)) = tiles_query.get(enemy.1) {
                        info!("Spawning enemy at {:?}", transform.translation);
                        commands.spawn((
                            Enemy::default(),
                            EnemyGizmo::default(),
                            Transform::from_translation(transform.translation),
                            TilePosition::for_entity(entity),
                            Movement::default(),
                            MovementSpeed::enemy(),
                            MovementState::Walking,
                            EnemyLastMovementTime::default(),
                            CharacterType::Enemy,
                        ));
                    }
                }
            }
            commands.spawn(EnemySpawned);
        }
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
    tiles_by_coord: &HashMap<(i32, i32), Entity>,
    player_tile_pos: (i32, i32),
    enemy_tile_positions: &[(i32, i32)],
    min_distance_from_player: f32,
    max_distance_from_player: f32,
    min_distance_from_enemies: f32,
) -> Option<((i32, i32), Entity)> {
    let mut rng = rand::rng();
    let player_vec = Vec2::new(player_tile_pos.0 as f32, player_tile_pos.1 as f32);

    // Prefiltruj najpierw odległość od gracza (najbardziej ograniczający warunek)
    let valid_tiles: Vec<_> = tiles_by_coord
        .iter()
        .filter(|(coord, _)| {
            let coord_vec = Vec2::new(coord.0 as f32, coord.1 as f32);
            let distance_to_player = coord_vec.distance(player_vec);

            // Szybkie odrzucenie: sprawdź zakres od gracza
            if distance_to_player < min_distance_from_player
                || distance_to_player > max_distance_from_player
            {
                return false;
            }

            // Sprawdź odległość od każdego wroga
            enemy_tile_positions.iter().all(|enemy_pos| {
                let enemy_vec = Vec2::new(enemy_pos.0 as f32, enemy_pos.1 as f32);
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

pub fn draw_enemy_gizmo(mut gizmos: Gizmos, enemy_query: Query<(&EnemyGizmo, &Transform)>) {
    for (gizmo, transform) in enemy_query.iter() {
        gizmos.sphere(transform.translation, gizmo.size, gizmo.color);
    }
}
