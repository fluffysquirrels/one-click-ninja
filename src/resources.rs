//! Shared resources

use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct Sounds {
    pub bass: Handle<AudioSource>,
    pub snare: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
    pub game_over_loop: Handle<AudioSource>,
    pub grunt: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub shield: Handle<AudioSource>,
    pub scream: Handle<AudioSource>,
    pub zombie_death: Handle<AudioSource>,
}

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
