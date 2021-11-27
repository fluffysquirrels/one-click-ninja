//! Shared components

use crate::types::{DamageType, Hp};
use std::time::Duration;

#[derive(Eq, PartialEq)]
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

pub struct AnimateSpriteSheet {
    pub frame_duration: Duration,
    pub next_frame_time: Duration,
    pub max_index: u32,
}
