use bevy::prelude::*;
use bevy_kira_audio::Audio;
use crate::{
    components::{AttackType, Enemy, Health, Player},
    enemy,
    events::Damage,
    events::{EnemyAttackTime, PlayerDefendAction},
    Icons,
    game_state::GameState,
    resources::Sounds,
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
            .insert_resource(PlayerDefend(false))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(show_fight_icons.system())
                    .with_system(hide_fight_icons.system())
                    .with_system(record_player_defend.system())
            );
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
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<(&Health, &AttackType), With<Enemy>>,
    audio: Res<Audio>,
    icons: Res<Icons>,
    mut player_defend: ResMut<PlayerDefend>,
    sounds: Res<Sounds>,
    time: Res<Time>,
) {
    for (enemy_health, enemy_attack_type) in enemy_query.single() {
        if enemy_health.current == 0 {
            return;
        }
        if let Some(_) = enemy_attack_time_reader.iter().next() {
            let did_defend = player_defend.0;
            player_defend.0 = false;
            if did_defend {
                commands.spawn_bundle(SpriteBundle {
                    material: icons.defend.clone(),
                    transform: Transform {
                        translation: Vec3::new(200., 100., 2.),
                        scale: Vec3::ONE * 0.15,
                        .. Default::default()
                    },
                    .. Default::default()
                })
                    .insert(FightIcon)
                    .insert(HideAfter { when: time.time_since_startup() + enemy::ATTACK_DURATION });
                audio.play(sounds.shield.clone());
            } else {
                // Didn't defend
                for player_entity in player_query.single() {
                    damage_writer.send(Damage {
                        target: player_entity,
                        hp: 1,
                        damage_type: enemy_attack_type.damage_type.clone(),
                    });
                }
            }

            commands.spawn_bundle(SpriteBundle {
                material: icons.attack.clone(),
                transform: Transform {
                    translation: Vec3::new(250., 100., 2.),
                    scale: Vec3::ONE * 0.15,
                    .. Default::default()
                },
                .. Default::default()
            })
                .insert(FightIcon)
                .insert(HideAfter { when: time.time_since_startup() + enemy::ATTACK_DURATION });
        }
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
