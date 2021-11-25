use bevy::prelude::*;
use crate::{
    gamestate::GameState,
};

/// Purely exists to move from Setup to Menu, giving other systems a chance to process resources
/// during on_enter of Setup.
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Setup)
                    .with_system(setup_done.system()));
    }
}

fn setup_done(mut state: ResMut<State<GameState>>) {
    info!("Setup done");
    state.set(GameState::Menu).unwrap();
}
