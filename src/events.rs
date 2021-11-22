//! Shared events

use bevy::prelude::*;
use crate::types::Hp;

pub struct PlayerAttackAction;
pub struct PlayerDefendAction;

pub struct EnemyAttackTime;

pub struct Damage {
    pub target: Entity,
    pub hp: Hp,
}

pub struct Die {
    pub target: Entity,
}
