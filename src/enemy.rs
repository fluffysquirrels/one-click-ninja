use bevy::prelude::*;
use crate::{
    components::{AttackType, Character, Enemy, Health},
    events::{Damage, EnemyAttackTime, PlayerAttackAction},
    resources::Fonts,
    types::{DamageType, Hp},
    game_state::GameState,
    loading,
};
use rand::Rng;
use std::time::Duration;

pub struct Plugin;

#[derive(Clone)]
struct CharacterSprites {
    idle: Handle<ColorMaterial>,
    attack: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
}

struct Sprites {
    archer: CharacterSprites,
    knight: CharacterSprites,
    mage: CharacterSprites,
}

struct AttackAnimation {
    until: std::time::Duration,
}

struct EnemyHpDisplay;

#[derive(Debug)]
struct RespawnTimer {
    at: std::time::Duration,
}

pub const ATTACK_DURATION: Duration = Duration::from_millis(300);
pub const START_HP: Hp = 2;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::CreateResources)
                    .with_system(create_resources.system()))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_current_enemy.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(enemy_attack.system())
                    .with_system(attack_animation.system())
                    .with_system(update_enemy_hp.system())
                    .with_system(enemy_was_attacked.system())
                    .with_system(respawn_timer.system())
            );
    }
}

fn create_resources(
    mut commands: Commands,
    texture_assets: Res<loading::TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Sprites {
        archer: CharacterSprites {
            idle: materials.add(texture_assets.archer_idle.clone().into()),
            attack: materials.add(texture_assets.archer_attack.clone().into()),
            dead: materials.add(texture_assets.archer_dead.clone().into()),
        },

        knight: CharacterSprites {
            idle: materials.add(texture_assets.knight_idle.clone().into()),
            attack: materials.add(texture_assets.knight_attack.clone().into()),
            dead: materials.add(texture_assets.knight_dead.clone().into()),
        },

        mage: CharacterSprites {
            idle: materials.add(texture_assets.mage_idle.clone().into()),
            attack: materials.add(texture_assets.mage_attack.clone().into()),
            dead: materials.add(texture_assets.mage_dead.clone().into()),
        }

    });
}

fn spawn_current_enemy(
    mut commands: Commands,
    sprites: Res<Sprites>,
    fonts: Res<Fonts>,
    enemy_query: Query<Entity, With<Enemy>>,
    enemy_hp_query: Query<Entity, With<EnemyHpDisplay>>,
) {
    for entity in enemy_query.single() {
        commands.entity(entity).despawn();
    }

    for entity in enemy_hp_query.single() {
        commands.entity(entity).despawn();
    }

    let character = match rand::thread_rng().gen_range(0..=2) {
        0 => Character::Archer,
        1 => Character::Knight,
        2 => Character::Mage,
        _ => unreachable!(),
    };
    let character_sprites = match character {
        Character::Archer => sprites.archer.clone(),
        Character::Knight => sprites.knight.clone(),
        Character::Mage => sprites.mage.clone(),
        Character::Player => unreachable!(),
    };

    commands.spawn_bundle(SpriteBundle {
        material: character_sprites.idle.clone(),
        transform: Transform {
            translation: Vec3::new(100., 200., 0.),
            scale: Vec3::ONE * 2.0,
            .. Default::default()
        },
        .. Default::default()
    })
        .insert(Enemy)
        .insert(character.clone())
        .insert(character_sprites)
        .insert(Health {
            current: START_HP,
            max: START_HP,
            vulnerable_to:
                match character {
                    Character::Archer => vec![DamageType::Arrow, DamageType::Magic],
                    Character::Knight => vec![DamageType::Magic],
                    Character::Mage   => vec![DamageType::Arrow, DamageType::Sword],
                    Character::Player => unreachable!(),
                }
            ,
        })
        .insert(AttackType { damage_type:
            match character {
                Character::Archer => DamageType::Arrow,
                Character::Knight => DamageType::Sword,
                Character::Mage   => DamageType::Magic,
                Character::Player => unreachable!(),
            }
        });

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            format_hp(START_HP, START_HP),
            TextStyle {
                font: fonts.default.clone(),
                font_size: 30.0,
                color: Color::RED,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        transform: Transform {
            translation: Vec3::new(100., 270., 0.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(EnemyHpDisplay);
}

fn respawn_current_enemy(
    mut commands: Commands,
    fonts: Res<Fonts>,
    sprites: Res<Sprites>,
    enemy_query: Query<Entity, With<Enemy>>,
    enemy_hp_query: Query<Entity, With<EnemyHpDisplay>>,
) {
    spawn_current_enemy(commands, sprites, fonts, enemy_query, enemy_hp_query);
}

fn enemy_attack(
    mut commands: Commands,
    mut enemy: Query<(Entity, &Health, &mut Handle<ColorMaterial>, &CharacterSprites),
                     With<Enemy>>,
    mut attack_time_reader: EventReader<EnemyAttackTime>,
    time: Res<Time>,
) {
    for (entity, health, mut material, sprites) in enemy.single_mut() {
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
    mut enemy: Query<(Entity, &mut Handle<ColorMaterial>, &AttackAnimation, &CharacterSprites),
                     With<Enemy>>,
) {
    for (entity, mut material, anim, sprites) in enemy.single_mut() {
        if time.time_since_startup() > anim.until {
            *material = sprites.idle.clone();
            commands.entity(entity).remove::<AttackAnimation>();
        }
    }
}

fn update_enemy_hp(
    mut commands: Commands,
    mut hp_display: Query<&mut Text, With<EnemyHpDisplay>>,
    mut enemy: Query<(Entity, &Health, &mut Handle<ColorMaterial>, &CharacterSprites),
                     With<Enemy>>,
    respawn_timer_query: Query<&RespawnTimer, With<Enemy>>,
    time: Res<Time>,
) {
    for (enemy_entity, health, mut material, sprites) in enemy.single_mut() {
        if health.current == 0 {
            *material = sprites.dead.clone();
            if !respawn_timer_query.single().is_ok() {
                commands.entity(enemy_entity)
                    .insert(RespawnTimer {
                        at: time.time_since_startup() + Duration::from_secs(2),
                    });
            }
        }
        for mut text in hp_display.single_mut() {
            text.sections[0].value = format_hp(health.current, health.max);
        }
    }
}

fn respawn_timer(
    commands: Commands,
    fonts: Res<Fonts>,
    sprites: Res<Sprites>,
    time: Res<Time>,
    respawn_query: Query<&RespawnTimer, With<Enemy>>,
    enemy_query: Query<Entity, With<Enemy>>,
    enemy_hp_query: Query<Entity, With<EnemyHpDisplay>>,
) {
    let timer = respawn_query.single();
    let timer = match timer.ok() {
        Some(t) => {
            t
        }
        None => return,
    };
    if time.time_since_startup() > timer.at {
        respawn_current_enemy(commands, fonts, sprites, enemy_query, enemy_hp_query);
    }
}

fn enemy_was_attacked(
    mut player_attack_reader: EventReader<PlayerAttackAction>,
    mut damage_writer: EventWriter<Damage>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if let Some(attack) = player_attack_reader.iter().next() {
        for enemy in enemy_query.single() {
            damage_writer.send(Damage {
                target: enemy,
                hp: 1,
                damage_type: attack.damage_type.clone(),
            });
        }
    }
}

fn format_hp(curr_hp: Hp, max_hp: Hp) -> String {
    format!("Enemy HP = {}/{}", curr_hp, max_hp)
}
