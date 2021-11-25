use bevy::prelude::*;
use crate::{
    gamestate::GameState,
};

/// Should have a menu but currently just goes directly to playing
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(setup_done.system()));
    }
}

fn setup_done(mut state: ResMut<State<GameState>>) {
    info!("Menu done");
    state.set(GameState::Playing).unwrap();
}
