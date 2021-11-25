use bevy::prelude::*;
use crate::{
    components::DespawnAfter,
    gamestate::GameState,
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(despawn_after.system()));
    }
}

fn despawn_after(
    mut commands: Commands,
    query: Query<(Entity, &DespawnAfter)>,
    time: Res<Time>,
) {
    let now = time.time_since_startup();
    for (entity, despawn_after) in query.iter() {
        if now > despawn_after.after {
            commands.entity(entity).despawn();
        }
    }
}
