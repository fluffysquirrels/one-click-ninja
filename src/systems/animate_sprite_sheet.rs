use bevy::prelude::*;
use crate::{
    components::AnimateSpriteSheet,
    game_state::GameState,
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(animate.system()));
    }
}

fn animate(
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimateSpriteSheet)>,
    time: Res<Time>,
) {
    for (mut sprite, mut anim) in query.iter_mut() {
        let now = time.time_since_startup();
        if now >= anim.next_frame_time {
            let next_index = sprite.index + 1;
            sprite.index =
                if anim.loop_ {
                    next_index % (anim.max_index + 1)
                } else {
                    // No loop
                    next_index.min(anim.max_index)
                };
            anim.next_frame_time = now + anim.frame_duration;
        }
    }
}
