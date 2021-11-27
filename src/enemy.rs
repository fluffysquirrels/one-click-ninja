use bevy::prelude::*;
use bevy_kira_audio::Audio;
use crate::{
    components::{AttackType, Character, DespawnAfter, Enemy, Health},
    events::{Damage, DamageApplied, EnemyAttackTime, PlayerAttackAction},
    loading::Sounds,
    types::DamageType,
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
    magic_ball: Handle<ColorMaterial>,
    health_background: Handle<ColorMaterial>,
    health_bar: Handle<ColorMaterial>,
}

struct AttackAnimation {
    until: std::time::Duration,
}

struct EnemyHpBackground;
struct EnemyHpBar;

#[derive(Debug)]
struct RespawnTimer {
    at: std::time::Duration,
}

pub const ATTACK_DURATION: Duration = Duration::from_millis(300);

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
                    .with_system(damage_applied.system())
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
        },

        magic_ball: materials.add(texture_assets.icon_magic.clone().into()),
        health_background: materials.add(texture_assets.health_enemy.clone().into()),
        health_bar: materials.add(Color::rgb(1.0, 0., 242./255.).into()),
    });
}

fn spawn_current_enemy(
    mut commands: Commands,
    sprites: Res<Sprites>,
    enemy_query: Query<Entity, With<Enemy>>,
    enemy_hp_bg_query: Query<Entity, With<EnemyHpBackground>>,
    enemy_hp_bar_query: Query<Entity, With<EnemyHpBar>>,
) {
    // Despawn any entities from previous runs
    for entity in enemy_query.single() {
        commands.entity(entity).despawn();
    }

    for entity in enemy_hp_bg_query.single() {
        commands.entity(entity).despawn();
    }

    for entity in enemy_hp_bar_query.single() {
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

    let start_hp = match character {
        Character::Archer => 3,
        Character::Knight => 4,
        Character::Mage => 2,
        Character::Player => unreachable!(),
    };

    let health = Health {
        current: start_hp,
        max: start_hp,
        vulnerable_to:
        match character {
            Character::Archer => vec![DamageType::Arrow, DamageType::Magic],
            Character::Knight => vec![DamageType::Magic],
            Character::Mage   => vec![DamageType::Arrow, DamageType::Sword],
            Character::Player => unreachable!(),
        }
        ,
    };
    commands.spawn_bundle(SpriteBundle {
        material: character_sprites.idle.clone(),
        transform: Transform {
            translation: Vec3::new(163., 173., 2.),
            scale: Vec3::ONE * 2.0,
            .. Default::default()
        },
        .. Default::default()
    })
        .insert(Enemy)
        .insert(character.clone())
        .insert(character_sprites)
        .insert(health.clone())
        .insert(AttackType { damage_type:
            match character {
                Character::Archer => DamageType::Arrow,
                Character::Knight => DamageType::Sword,
                Character::Mage   => DamageType::Magic,
                Character::Player => unreachable!(),
            }
        });

    commands.spawn_bundle(SpriteBundle {
        material: sprites.health_background.clone(),
        transform: Transform {
            translation: Vec3::new(163., 243., 3.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(EnemyHpBackground);

    commands.spawn_bundle(SpriteBundle {
        material: sprites.health_bar.clone(),
        sprite: Sprite::new(Vec2::new(1.0, 1.0)),
        transform: health_bar_transform(&health),
        .. Default::default()
    }).insert(EnemyHpBar);
}

fn respawn_current_enemy(
    commands: Commands,
    sprites: Res<Sprites>,
    enemy_query: Query<Entity, With<Enemy>>,
    enemy_hp_bg_query: Query<Entity, With<EnemyHpBackground>>,
    enemy_hp_bar_query: Query<Entity, With<EnemyHpBar>>,
) {
    spawn_current_enemy(commands, sprites, enemy_query, enemy_hp_bg_query,
                        enemy_hp_bar_query);
}

fn enemy_attack(
    mut commands: Commands,
    mut enemy: Query<(Entity, &Health, &mut Handle<ColorMaterial>, &CharacterSprites,
                      &AttackType),
                     With<Enemy>>,
    mut attack_time_reader: EventReader<EnemyAttackTime>,
    sprites: Res<Sprites>,
    time: Res<Time>,
) {
    for (entity, health, mut material, char_sprites, attack_type) in enemy.single_mut() {
        if health.current > 0 {
            if attack_time_reader.iter().next().is_some() {
                commands.entity(entity).insert(AttackAnimation {
                    until: time.time_since_startup() + ATTACK_DURATION,
                });
                *material = char_sprites.attack.clone();
                if attack_type.damage_type == DamageType::Magic {
                    commands.spawn()
                        .insert(DespawnAfter {
                            after: time.time_since_startup() + Duration::from_millis(300),
                        })
                        .insert_bundle(SpriteBundle {
                            material: sprites.magic_ball.clone(),
                            transform: Transform {
                                translation: Vec3::new(163., 223., 5.),
                                scale: Vec3::ONE * 0.25,
                                .. Default::default()
                            },
                            .. Default::default()
                        });
                }
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
    mut hp_bar: Query<&mut Transform, With<EnemyHpBar>>,
    mut enemy: Query<(Entity, &Health, &mut Handle<ColorMaterial>, &CharacterSprites),
                     With<Enemy>>,
    respawn_timer_query: Query<&RespawnTimer, With<Enemy>>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
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
                audio.play(sounds.zombie_death.clone());
            }
        }
         for mut hp_transform in hp_bar.single_mut() {
             *hp_transform = health_bar_transform(health);
         }
    }
}

/// TODO: This is O(n) in number of enemies, seems inefficient.
fn damage_applied(
    mut damage_applied_reader: EventReader<DamageApplied>,
    enemy_query: Query<&Enemy>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    for damage_applied in damage_applied_reader.iter() {
        if let Ok(_enemy) = enemy_query.get(damage_applied.damage.target) {
            audio.play(sounds.hit.clone());
        }
    }
}


fn respawn_timer(
    commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    respawn_query: Query<&RespawnTimer, With<Enemy>>,
    enemy_query: Query<Entity, With<Enemy>>,
    enemy_hp_bg_query: Query<Entity, With<EnemyHpBackground>>,
    enemy_hp_bar_query: Query<Entity, With<EnemyHpBar>>,
) {
    let timer = respawn_query.single();
    let timer = match timer.ok() {
        Some(t) => {
            t
        }
        None => return,
    };
    if time.time_since_startup() > timer.at {
        respawn_current_enemy(commands, sprites, enemy_query, enemy_hp_bg_query,
                              enemy_hp_bar_query);
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

fn health_bar_transform(health: &Health) -> Transform {
    assert!(health.max >= 1);
    let portion = (health.current as f32) / (health.max as f32);
    let width_pixels = portion * 162.;

    Transform {
        translation: Vec3::new(163. - (162. / 2.) + width_pixels / 2., 243., 4.),
        scale: Vec3::new(width_pixels, 16., 1.),
        .. Default::default()
    }
}
