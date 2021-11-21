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

struct PlayerDefend(bool);

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(show_fight_icons.system())
            .add_system(hide_fight_icons.system())
            .add_system(record_player_defend.system())
            .insert_resource(PlayerDefend(false));
    }
}

fn record_player_defend(
    mut player_defend_action: EventReader<PlayerDefendAction>,
    mut player_defend: ResMut<PlayerDefend>,
) {
    player_defend.0 = player_defend.0 || player_defend_action.iter().next().is_some();
}

fn show_fight_icons(
    mut commands: Commands,
    icons: Res<Icons>,
    mut enemy_attack_time_reader: EventReader<EnemyAttackTime>,
    time: Res<Time>,
    mut player_defend: ResMut<PlayerDefend>,
) {
    if enemy_attack_time_reader.iter().next().is_some() {
        if player_defend.0 {
            commands.spawn_bundle(SpriteBundle {
                material: icons.defend.clone(),
                transform: Transform {
                    translation: Vec3::new(200., 100., 0.),
                    scale: Vec3::ONE * 0.15,
                    .. Default::default()
                },
                .. Default::default()
            })
                .insert(FightIcon)
                .insert(HideAfter { when: time.time_since_startup() + enemy::ATTACK_DURATION });

            player_defend.0 = false;
        }

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
    mut player_defend: ResMut<PlayerDefend>,
) {
    for (entity, _, hide_after) in hide_query.iter() {
        if time.time_since_startup() > hide_after.when {
            commands.entity(entity).despawn();
        }
    }
}
