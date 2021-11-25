//! Shared resources

use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct Sounds {
    pub bass: Handle<AudioSource>,
    pub snare: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
}

pub struct Icons {
    pub attack: Handle<ColorMaterial>,
    pub defend: Handle<ColorMaterial>,
}

pub struct Fonts {
    pub default: Handle<Font>,
}
