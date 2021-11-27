use bevy::prelude::*;
use crate::{
    game_state::GameState,
    loading::CountdownTextures,
    resources::Countdown,
};
use std::time::Duration;

pub struct Plugin;

struct CountdownSprite {
    number: u8,
    next_count: Duration,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(show_countdown.system()))
            ;
    }
}

fn setup(
    mut commands: Commands,
    tex: Res<CountdownTextures>,
    time: Res<Time>,
) {
    commands.insert_resource(Countdown::Counting);

    commands.spawn_bundle(SpriteBundle {
        material: tex.number_3.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 5.),
            scale: Vec3::ONE,
            .. Default::default()
        },
        .. Default::default()
    }).insert(CountdownSprite {
        number: 3,
        next_count: time.time_since_startup() + Duration::from_secs(1),
    });
}

fn show_countdown(
    mut commands: Commands,
    mut sprite: Query<(Entity, &mut Handle<ColorMaterial>, &mut CountdownSprite)>,
    mut countdown_res: ResMut<Countdown>,
    tex: Res<CountdownTextures>,
    time: Res<Time>,
) {
    for (entity, mut mat, mut count) in sprite.single_mut() {
        let now = time.time_since_startup();
        if now > count.next_count {
            let next_number = count.number.checked_sub(1);
            match next_number {
                None => {
                    commands.entity(entity).despawn();
                    *countdown_res = Countdown::Disabled;
                },
                Some(n) => {
                    count.number = n;
                    count.next_count = count.next_count + Duration::from_secs(1);
                    *mat = match n {
                        2 => tex.number_2.clone(),
                        1 => tex.number_1.clone(),
                        0 => tex.fight.clone(),
                        _ => unreachable!(),
                    };
                }
            }
        }
    }
}
