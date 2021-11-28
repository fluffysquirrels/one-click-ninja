use bevy::prelude::*;
use bevy_kira_audio::Audio;
use crate::{
    components::{AnimateSpriteSheet, AttackType, Character, DespawnAfter, Enemy, Health},
    events::{Damage, DamageApplied, EnemyAttackTime, PlayerAttackAction},
    game_state::GameState,
    loading::{self, Fonts, Sounds},
    resources::Level,
    types::DamageType,
};
use rand::Rng;
use std::time::Duration;

pub struct Plugin;

#[derive(Clone)]
struct CharacterSprites {
    idle: Handle<TextureAtlas>,
    attack: Handle<TextureAtlas>,
    death: Handle<TextureAtlas>,
}

struct Sprites {
    archer: CharacterSprites,
    knight: CharacterSprites,
    mage: CharacterSprites,
    boss: CharacterSprites,
    magic_ball: Handle<ColorMaterial>,
    ray: Handle<TextureAtlas>,
    health_background: Handle<ColorMaterial>,
    health_bar: Handle<ColorMaterial>,
    boss_text: Handle<ColorMaterial>,
    win_text: Handle<ColorMaterial>,
}

struct AttackAnimation {
    until: std::time::Duration,
}

struct HpBackground;
struct HpBar;
struct LevelText;

struct EnemyEntity;

#[derive(Debug)]
struct RespawnTimer {
    at: std::time::Duration,
}

pub const ATTACK_DURATION: Duration = Duration::from_millis(300);
pub const NUM_MOB_LEVELS: u8 = 5;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::CreateResources)
                    .with_system(create_resources.system())
                    .with_system(insert_level.system()))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(set_level.system().label("set_level"))
                    .with_system(spawn_current_enemy.system().after("set_level")))
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(Sprites {
        archer: CharacterSprites {
            idle: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.archer_idle.clone(),
                                        Vec2::new(64., 64.),
                                        1, // columns
                                        1  // rows
                                        )),
            attack: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.archer_attack.clone(),
                                        Vec2::new(64., 64.),
                                        1, // columns
                                        1  // rows
                                        )),
            death: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.archer_dead.clone(),
                                        Vec2::new(64., 64.),
                                        1, // columns
                                        1  // rows
                                        )),
        },

        knight: CharacterSprites {
            idle: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.knight_idle.clone(),
                                        Vec2::new(64., 64.),
                                        1, // columns
                                        1  // rows
                                        )),
            attack: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.knight_attack.clone(),
                                        Vec2::new(64., 64.),
                                        1, // columns
                                        1  // rows
                                        )),
            death: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.knight_dead.clone(),
                                        Vec2::new(64., 64.),
                                        1, // columns
                                        1  // rows
                                        )),
        },

        mage: CharacterSprites {
            idle: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.mage_idle.clone(),
                                        Vec2::new(64., 64.),
                                        1, // columns
                                        1  // rows
                                        )),
            attack: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.mage_attack.clone(),
                                        Vec2::new(64., 64.),
                                        1, // columns
                                        1  // rows
                                        )),
            death: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.mage_dead.clone(),
                                        Vec2::new(64., 64.),
                                        1, // columns
                                        1  // rows
                                        )),
        },

        boss: CharacterSprites {
            idle: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.boss_idle.clone(),
                                        Vec2::new(65., 54.),
                                        1, // columns
                                        1  // rows
                                        )),
            attack: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.boss_attack_sheet.clone(),
                                        Vec2::new(65., 54.),
                                        4, // columns
                                        1  // rows
                                        )),
            death: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.boss_death_sheet.clone(),
                                        Vec2::new(65., 54.),
                                        5, // columns
                                        1  // rows
                                        )),
        },

        ray: texture_atlases.add(
                TextureAtlas::from_grid(texture_assets.boss_ray_sheet.clone(),
                                        Vec2::new(34., 140.),
                                        6, // columns
                                        1  // rows
                                        )),
        magic_ball: materials.add(texture_assets.icon_magic.clone().into()),
        health_background: materials.add(texture_assets.health_enemy.clone().into()),
        health_bar: materials.add(Color::rgb(1.0, 0., 242./255.).into()),
        boss_text: materials.add(texture_assets.boss_text.clone().into()),
        win_text: materials.add(texture_assets.win_text.clone().into()),
    });
}

// It's weird needing two systems to insert the resource, but I get resource not found
// if I just insert on entering GameState::Playing
fn insert_level(
    mut commands: Commands,
) {
    commands.insert_resource(
        Level::Mob(1)
    );
}

fn set_level(
    mut level: ResMut<Level>,
) {
    *level = Level::Mob(1);
}

fn spawn_current_enemy(
    mut commands: Commands,
    despawn_query: Query<Entity, With<EnemyEntity>>,
    fonts: Res<Fonts>,
    level: ResMut<Level>,
    sprites: Res<Sprites>,
) {
    // Despawn any entities from previous runs
    for entity in despawn_query.iter() {
        commands.entity(entity).despawn();
    }

    let character = match *level {
        Level::Mob(_mob_level) => match rand::thread_rng().gen_range(0..=2) {
            0 => Character::Archer,
            1 => Character::Knight,
            2 => Character::Mage,
            _ => unreachable!(),
        },
        Level::Boss => Character::Boss,
    };
    let character_sprites: CharacterSprites = match character {
        Character::Archer => sprites.archer.clone(),
        Character::Knight => sprites.knight.clone(),
        Character::Mage =>   sprites.mage.clone(),
        Character::Boss =>   sprites.boss.clone(),
        Character::Player => unreachable!(),
    };

    let start_hp = match character {
        Character::Archer => 3,
        Character::Knight => 4,
        Character::Mage => 2,
        Character::Boss => 10,
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
            Character::Boss   => vec![DamageType::Magic, DamageType::Sword],
            Character::Player => unreachable!(),
        }
        ,
    };
    commands.spawn()
        .insert(Enemy)
        .insert(EnemyEntity)
        .insert(character.clone())
        .insert(character_sprites.clone())
        .insert(health.clone())
        .insert(AttackType { damage_type:
            match character {
                Character::Archer => DamageType::Arrow,
                Character::Knight => DamageType::Sword,
                Character::Mage   => DamageType::Magic,
                Character::Boss   => DamageType::Ray,
                Character::Player => unreachable!(),
            }
        })
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                .. Default::default()
            },
            texture_atlas: character_sprites.idle.clone(),
            transform: Transform {
                translation: Vec3::new(163.,
                                       match character {
                                           Character::Boss => 150.,
                                           _ => 173.
                                       },
                                       2.),
                scale: Vec3::ONE * (match character {
                    Character::Boss => 3.,
                    _ => 2.,
                }),
                .. Default::default()
            },
            .. Default::default()
        })
        .insert(AnimateSpriteSheet::never());

    // Spawn HP bar
    commands.spawn_bundle(SpriteBundle {
        material: sprites.health_background.clone(),
        transform: Transform {
            translation: Vec3::new(163., 243., 3.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(HpBackground)
      .insert(EnemyEntity);


    commands.spawn_bundle(SpriteBundle {
        material: sprites.health_bar.clone(),
        sprite: Sprite::new(Vec2::new(1.0, 1.0)),
        transform: health_bar_transform(&health),
        .. Default::default()
    }).insert(HpBar)
      .insert(EnemyEntity);


    // Spawn level text
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            match *level {
                Level::Mob(n) => format!("Level {} of {}", n, NUM_MOB_LEVELS + 1),
                Level::Boss => "Boss Level!".to_owned(),
            },
            TextStyle {
                font: fonts.fiendish.clone(),
                font_size: 30.,
                color: Color::rgb(242./255., 0., 48./255.),
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }),
        transform: Transform {
            translation: Vec3::new(-300., 250., 5.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(LevelText)
      .insert(EnemyEntity);
}

fn enemy_attack(
    mut commands: Commands,
    mut enemy: Query<(Entity, &Health, &mut Handle<TextureAtlas>, &mut TextureAtlasSprite,
                      &mut AnimateSpriteSheet, &CharacterSprites, &AttackType),
                     With<Enemy>>,
    mut attack_time_reader: EventReader<EnemyAttackTime>,
    sprites: Res<Sprites>,
    atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    for (entity, health, mut atlas, mut sprite, mut anim, char_sprites, attack_type) in
        enemy.single_mut()
    {
        if health.current > 0 {
            if attack_time_reader.iter().next().is_some() {
                commands.entity(entity).insert(AttackAnimation {
                    until: time.time_since_startup() + ATTACK_DURATION,
                });
                *atlas = char_sprites.attack.clone();
                sprite.index = 0;
                *anim = AnimateSpriteSheet {
                    frame_duration: Duration::from_millis(200),
                    next_frame_time: time.time_since_startup() + Duration::from_millis(200),
                    max_index: atlases.get(atlas.clone())
                                      .map(|a| a.len() - 1).unwrap_or(0) as u32,
                };
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
    mut enemy: Query<(Entity, &mut Handle<TextureAtlas>, &mut TextureAtlasSprite,
                      &mut AnimateSpriteSheet, &AttackAnimation, &CharacterSprites),
                     With<Enemy>>,
) {
    for (entity, mut atlas, mut sprite, mut anim, attack_animation, char_sprites)
        in enemy.single_mut()
    {
        if time.time_since_startup() > attack_animation.until {
            *atlas = char_sprites.idle.clone();
            sprite.index = 0;
            *anim = AnimateSpriteSheet::never();
            commands.entity(entity).remove::<AttackAnimation>();
        }
    }
}

fn update_enemy_hp(
    mut commands: Commands,
    mut hp_bar: Query<&mut Transform, With<HpBar>>,
    mut enemy: Query<(Entity, &Health, &mut Handle<TextureAtlas>, &mut TextureAtlasSprite,
                      &mut AnimateSpriteSheet, &CharacterSprites),
                     With<Enemy>>,
    respawn_timer_query: Query<&RespawnTimer, With<Enemy>>,
    audio: Res<Audio>,
    level: Res<Level>,
    sounds: Res<Sounds>,
    sprites: Res<Sprites>,
    atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    for (enemy_entity, health, mut atlas, mut sprite, mut anim, character_sprites)
        in enemy.single_mut()
        if health.current == 0 {
            *atlas = character_sprites.death.clone();
            if !respawn_timer_query.single().is_ok() {
                // Just died.
                sprite.index = 0;
                *anim = AnimateSpriteSheet {
                    frame_duration: Duration::from_millis(200),
                    next_frame_time: time.time_since_startup() + Duration::from_millis(200),
                    max_index: atlases.get(atlas.clone())
                        .map(|a| a.len() - 1).unwrap_or(0) as u32,
                };
                let boss_next = *level == Level::Mob(NUM_MOB_LEVELS);
                let boss_done = *level == Level::Boss;
                if boss_done {
                    audio.play(sounds.zombie_death.clone());
                    commands.spawn_bundle(SpriteBundle {
                        material: sprites.win_text.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., 0., 5.),
                            scale: Vec3::ONE,
                            .. Default::default()
                        },
                        .. Default::default()
                    }).insert(DespawnAfter {
                        after: time.time_since_startup() + Duration::from_secs(5),
                    });
                } else if boss_next {
                    audio.play(sounds.boss_intro.clone());
                    commands.spawn_bundle(SpriteBundle {
                        material: sprites.boss_text.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., 0., 5.),
                            scale: Vec3::ONE,
                            .. Default::default()
                        },
                        .. Default::default()
                    }).insert(DespawnAfter {
                        after: time.time_since_startup() + Duration::from_secs(4),
                    });
                } else {
                    // Just a regular level
                    audio.play(sounds.zombie_death.clone());
                }

                commands.entity(enemy_entity)
                    .insert(RespawnTimer {
                        at: time.time_since_startup() +
                            if boss_next || boss_done {
                                Duration::from_secs(5)
                            } else {
                                Duration::from_secs(2)
                            }
                    });
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
    respawn_query: Query<&RespawnTimer, With<Enemy>>,
    despawn_query: Query<Entity, With<EnemyEntity>>,
    fonts: Res<Fonts>,
    mut level: ResMut<Level>,
    sprites: Res<Sprites>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
) {
    if let Ok(timer) = respawn_query.single() {
        if time.time_since_startup() > timer.at {
            if *level == Level::Boss {
                state.set(GameState::Menu).unwrap();
                return;
            }
            *level = match *level {
                Level::Mob(n) if n == NUM_MOB_LEVELS => Level::Boss,
                Level::Mob(n) => {
                    assert!(n < u8::MAX);
                    Level::Mob(n + 1)
                },
                Level::Boss => unreachable!(),
            };
            spawn_current_enemy(commands, despawn_query, fonts, level, sprites);
        }
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
    const WIDTH: f32 = 162.;
    let width_pixels = portion * WIDTH;
    let left_edge = 163. - (WIDTH / 2.);
    Transform {
        translation: Vec3::new(left_edge + width_pixels / 2., 243., 4.),
        scale: Vec3::new(width_pixels, 16., 1.),
        .. Default::default()
    }
}
