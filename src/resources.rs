//! Shared resources

use bevy::prelude::*;

pub struct Sounds {
    pub bass: Handle<AudioSource>,
    pub snare: Handle<AudioSource>,
}

pub struct Icons {
    pub attack: Handle<ColorMaterial>,
    pub defend: Handle<ColorMaterial>,
}
