//! Shared components

// use bevy::prelude::*;
use crate::types::Hp;

pub enum Action {
    Attack,
    Defend,
}

pub struct Enemy;

pub struct Health {
    pub current: Hp,
    pub max: Hp,
}

pub struct Player;
