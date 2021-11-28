//! Shared resources

use bevy::prelude::*;

pub struct Icons {
    pub attack: Handle<ColorMaterial>,
    pub defend: Handle<ColorMaterial>,
}

pub struct Fonts {
    pub default: Handle<Font>,
}

#[derive(Eq, PartialEq)]
pub enum Countdown {
    Disabled,
    Counting,
}

#[derive(Eq, PartialEq)]
pub enum Level {
    /// 1 indexed mob level
    Mob(u8),
    Boss
}
