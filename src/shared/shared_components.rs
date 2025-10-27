use bevy::prelude::*;


#[derive(Component, Eq, PartialEq)]
pub enum CharacterType {
    Player,
    Enemy,
}