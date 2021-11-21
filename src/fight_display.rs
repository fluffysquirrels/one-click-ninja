use bevy::prelude::*;
use crate::{
    components::{EnemyAttackTime, PlayerAttackAction, PlayerDefendAction},
    enemy,
    Icons,
};
use std::time::Duration;

pub struct Plugin;

struct FightIcon;
struct HideAfter {
    when: Duration,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(show_fight_icons.system())
            .add_system(hide_fight_icons.system());
    }
}

fn show_fight_icons(
    mut commands: Commands,
    icons: Res<Icons>,
    mut enemy_attack_time_reader: EventReader<EnemyAttackTime>,
    time: Res<Time>,
) {
    if enemy_attack_time_reader.iter().next().is_some() {
        commands.spawn_bundle(SpriteBundle {
            material: icons.attack.clone(),
            transform: Transform {
                translation: Vec3::new(250., 100., 0.),
                scale: Vec3::ONE * 0.15,
                .. Default::default()
            },
            .. Default::default()
        })
            .insert(FightIcon)
            .insert(HideAfter { when: time.time_since_startup() + enemy::ATTACK_DURATION });
    }
}

fn hide_fight_icons(
    mut commands: Commands,
    time: Res<Time>,
    hide_query: Query<(Entity, &FightIcon, &HideAfter)>,
) {
    for (entity, _, hide_after) in hide_query.iter() {
        if time.time_since_startup() > hide_after.when {
            commands.entity(entity).despawn();
        }
    }
}
