use bevy::prelude::*;
use crate::{
    components::Player,
    enemy,
    events::Damage,
    events::{EnemyAttackTime, PlayerAttackAction, PlayerDefendAction},
    Icons,
};
use std::time::Duration;

pub struct Plugin;

struct FightIcon;
struct HideAfter {
    when: Duration,
}

/// Is the player defending this beat?
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

/// TODO: This combines display and logic, should probably decouple these.
fn show_fight_icons(
    mut commands: Commands,
    mut enemy_attack_time_reader: EventReader<EnemyAttackTime>,
    mut damage_writer: EventWriter<Damage>,
    player_query: Query<(Entity, &Player)>,
    icons: Res<Icons>,
    mut player_defend: ResMut<PlayerDefend>,
    time: Res<Time>,
) {
    if enemy_attack_time_reader.iter().next().is_some() {
        let did_defend = player_defend.0;
        player_defend.0 = false;
        if did_defend {
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
        } else {
            // Didn't defend
            for (player_entity, _) in player_query.single() {
                damage_writer.send(Damage {
                    target: player_entity,
                    hp: 1,
                });
            }
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
) {
    for (entity, _, hide_after) in hide_query.iter() {
        if time.time_since_startup() > hide_after.when {
            commands.entity(entity).despawn();
        }
    }
}
