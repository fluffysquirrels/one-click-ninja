use bevy::prelude::*;
use crate::{
    components::{Enemy, Health},
    events::{Damage, EnemyAttackTime, PlayerAttackAction},
    types::Hp,
};
use std::time::Duration;

pub struct Plugin;

struct Sprites {
    idle: Handle<ColorMaterial>,
    attack: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
}

struct AttackAnimation {
    until: std::time::Duration,
}

struct EnemyHpDisplay;

pub const ATTACK_DURATION: Duration = Duration::from_millis(300);
pub const START_HP: Hp = 2;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_resources.system())
            .add_startup_system(spawn_current_enemy.system())
            .add_system(enemy_attack.system())
            .add_system(attack_animation.system())
            .add_system(update_enemy_hp.system())
            .add_system(enemy_was_attacked.system());
    }
}

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Sprites {
        idle: materials.add(
            asset_server.load("sprites/lpc-medieval-fantasy-character/our_work/archer/walk_down/00.png").into()),
        attack: materials.add(
            asset_server.load("sprites/lpc-medieval-fantasy-character/our_work/archer/spear_down/05.png").into()),
        dead: materials.add(
            asset_server.load("sprites/lpc-medieval-fantasy-character/our_work/archer/die/05.png").into()),
    });
}

fn spawn_current_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    sprites: Res<Sprites>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: sprites.idle.clone(),
        transform: Transform {
            translation: Vec3::new(200., 200., 0.),
            scale: Vec3::ONE * 2.0,
            .. Default::default()
        },
        .. Default::default()
    })
        .insert(Enemy)
        .insert(Health {
            current: START_HP,
            max: START_HP,
        });

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            format_hp(START_HP, START_HP),
            TextStyle {
                // TODO: Stash this in a resource
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::RED,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        transform: Transform {
            translation: Vec3::new(200., 150., 0.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(EnemyHpDisplay);
}

fn enemy_attack(
    mut commands: Commands,
    mut enemy: Query<(Entity, &Health, &mut Handle<ColorMaterial>), With<Enemy>>,
    mut attack_time_reader: EventReader<EnemyAttackTime>,
    sprites: Res<Sprites>,
    time: Res<Time>,
) {
    for (entity, health, mut material) in enemy.single_mut() {
        if health.current > 0 {
            if attack_time_reader.iter().next().is_some() {
                commands.entity(entity).insert(AttackAnimation {
                    until: time.time_since_startup() + ATTACK_DURATION,
                });
                *material = sprites.attack.clone();
            }
        }
    }
}

fn attack_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy: Query<(Entity, &mut Handle<ColorMaterial>, &AttackAnimation), With<Enemy>>,
    sprites: Res<Sprites>,
) {
    for (entity, mut material, anim) in enemy.single_mut() {
        if time.time_since_startup() > anim.until {
            *material = sprites.idle.clone();
            commands.entity(entity).remove::<AttackAnimation>();
        }
    }
}

fn update_enemy_hp(
    mut hp_display: Query<&mut Text, With<EnemyHpDisplay>>,
    mut enemy: Query<(&Health, &mut Handle<ColorMaterial>), With<Enemy>>,
    sprites: Res<Sprites>,
) {
    for (health, mut material) in enemy.single_mut() {
        if health.current == 0 {
            *material = sprites.dead.clone();
        }
        for mut text in hp_display.single_mut() {
            text.sections[0].value = format_hp(health.current, health.max);
        }
    }
}

fn enemy_was_attacked(
    mut player_attack_reader: EventReader<PlayerAttackAction>,
    mut damage_writer: EventWriter<Damage>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if player_attack_reader.iter().next().is_some() {
        for enemy in enemy_query.single() {
            damage_writer.send(Damage {
                target: enemy,
                hp: 1,
            });
        }
    }
}

fn format_hp(curr_hp: Hp, max_hp: Hp) -> String {
    format!("Enemy HP = {}/{}", curr_hp, max_hp)
}
