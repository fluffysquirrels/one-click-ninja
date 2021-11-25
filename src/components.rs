//! Shared components

use crate::types::{DamageType, Hp};
use std::time::Duration;

pub enum Action {
    AttackArrow,
    AttackMagic,
    AttackSword,
    Defend,
}

pub struct Enemy;

pub struct Health {
    pub current: Hp,
    pub max: Hp,
    pub vulnerable_to: Vec<DamageType>,
}

pub struct Player;

#[derive(Clone)]
pub enum Character {
    Archer,
    Knight,
    Mage,

    #[allow(dead_code)]
    Player,
}

pub struct AttackType {
    pub damage_type: DamageType,
}

pub struct DespawnAfter {
    pub after: Duration,
}
