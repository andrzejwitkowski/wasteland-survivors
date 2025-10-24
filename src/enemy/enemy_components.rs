use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub max_health: i32,
    pub speed: f32,
    pub attack_damage: i32,
    pub attack_range: f32,
}

#[derive(Component)]
pub struct EnemyGizmo {
    pub color: Color, 
    pub size: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: 100,
            max_health: 100,
            speed: 10.0,
            attack_damage: 10,
            attack_range: 10.0,
        }
    }
}

impl Default for EnemyGizmo {
    fn default() -> Self {
        Self {
            color: Color::srgb(0.0, 0.0, 1.0),
            size: 1.0,
        }
    }
}