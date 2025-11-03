use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::{TilePosition, MovementState};
use crate::shared::CharacterType;

/// System rozwiązywania kolizji w czasie ruchu (Diablo-style).
///
/// Jeśli wiele jednostek próbuje przejść na ten sam kafelek jednocześnie:
/// - Pierwsza jednostka wykonuje ruch
/// - Reszta czeka (MovementState::Idle) i spróbuje ponownie później
pub fn resolve_movement_collisions(
    mut characters: Query<(Entity, &TilePosition, &mut MovementState), With<CharacterType>>,
) {
    // Zbierz kto próbuje stanąć na jakim kafelku
    let mut tile_claims: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity, tile_pos, state) in characters.iter() {
        // Tylko jednostki które się poruszają
        if *state == MovementState::Walking {
            if let Some(target_tile) = tile_pos.tile {
                tile_claims
                    .entry(target_tile)
                    .or_default()
                    .push(entity);
            }
        }
    }

    // Jeśli więcej niż jedna jednostka chce ten sam kafelek
    for (_tile, claimants) in tile_claims.iter() {
        if claimants.len() > 1 {
            info!(
                "Collision detected: {} units trying to move to same tile",
                claimants.len()
            );

            // Pierwsza idzie, reszta czeka
            for (i, &entity) in claimants.iter().enumerate() {
                if i > 0 {
                    if let Ok((_, _, mut state)) = characters.get_mut(entity) {
                        *state = MovementState::Idle;
                        info!("Unit {:?} waiting due to collision", entity);
                    }
                }
            }
        }
    }
}

/// System zapobiegający blokowaniu jednostek przez siebie nawzajem.
///
/// Jeśli jednostka nie może znaleźć ścieżki (bo jest otoczona),
/// może spróbować "przepchnąć" inną jednostkę.
#[allow(dead_code)]
pub fn push_blocking_units(
    // TODO: Opcjonalna implementacja
    // Jednostki mogą "przepychać" inne jeśli są zablokowane
) {
    // Zaawansowana funkcjonalność - na razie nieimplementowana
    // W Diablo jednostki po prostu czekają lub próbują znaleźć alternatywną ścieżkę
}

