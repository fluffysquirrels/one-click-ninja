//! Shared components

use crate::types::{DamageType, Hp};

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
    Player,
}

pub struct AttackType {
    pub damage_type: DamageType,
}
