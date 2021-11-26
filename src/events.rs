//! Shared events

use bevy::prelude::*;
use crate::types::{DamageType, Hp};

#[derive(Debug)]
pub struct PlayerAttackAction {
    pub damage_type: DamageType,
}
pub struct PlayerDefendAction;

pub struct EnemyAttackTime;

pub struct Damage {
    pub target: Entity,
    pub hp: Hp,
    pub damage_type: DamageType,
}

pub struct Die {
    pub target: Entity,
}

#[derive(Debug)]
pub struct MusicTime {
    pub loop_position: f64,
    pub beat_in_bar: f64,
}
