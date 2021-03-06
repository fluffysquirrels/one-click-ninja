use bevy::prelude::*;
use bevy_kira_audio::Audio;
use crate::{
    components::{AnimateSpriteSheet, DespawnAfter, Health, Player},
    events::{DamageApplied, PlayerAttackAction, PlayerDefendAction},
    loading::Sounds,
    types::{DamageType, Hp},
    game_state::GameState,
    loading,
};
use std::time::Duration;

pub struct Plugin;

struct HpBackground;
struct HpBar;

const START_HP: Hp = 10;

struct Sprites {
    idle: Handle<ColorMaterial>,
    attack_arrow: Handle<ColorMaterial>,
    attack_magic: Handle<ColorMaterial>,
    attack_sword: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
    magic_ball: Handle<ColorMaterial>,
    blood_splatter: Handle<TextureAtlas>,
    health_background: Handle<ColorMaterial>,
    health_bar: Handle<ColorMaterial>,
    shield_flash: Handle<TextureAtlas>,
}

enum AnimationState {
    Dead {
        until: Duration,
    },
    Attacking {
        until: Duration,
        damage_type: DamageType
    },
    Idle,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::CreateResources)
                    .with_system(create_resources.system()))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_player_hp.system())
                    .with_system(spawn_player.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_player_display.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_attack_visuals.system())
                    .with_system(player_defend_visuals.system())
                    .with_system(player_damage_applied.system())
                    .with_system(die.system()))
            ;
    }
}

fn create_resources(
    mut commands: Commands,
    texture_assets: Res<loading::TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(Sprites {
        idle: materials.add(texture_assets.player_idle.clone().into()),
        attack_arrow: materials.add(texture_assets.player_attack_arrow.clone().into()),
        attack_sword: materials.add(texture_assets.player_attack_sword.clone().into()),
        attack_magic: materials.add(texture_assets.player_attack_magic.clone().into()),
        dead: materials.add(texture_assets.player_dead.clone().into()),
        magic_ball: materials.add(texture_assets.icon_magic.clone().into()),
        blood_splatter: texture_atlases.add(
            TextureAtlas::from_grid(texture_assets.blood_splatter.clone(),
                                    Vec2::new(70., 51.),
                                    6, // columns
                                    1  // rows
                                    )),
        health_background: materials.add(texture_assets.health_player.clone().into()),
        health_bar: materials.add(Color::rgb(64./255., 1., 0.).into()),
        shield_flash: texture_atlases.add(
            TextureAtlas::from_grid(texture_assets.shield_flash_sheet.clone(),
                                    Vec2::new(280., 420.),
                                    4, // columns
                                    1  // rows
                                    ))
    });
}

fn spawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    sprites: Res<Sprites>,
) {
    for ent in player_query.iter() {
        commands.entity(ent).despawn();
    }

    commands
        .spawn()
        .insert(Player)
        .insert(player_start_health())
        .insert(AnimationState::Idle)
        .insert_bundle(SpriteBundle {
            material: sprites.idle.clone(),
            transform: Transform {
                translation: Vec3::new(163., -27., 3.),
                scale: Vec3::ONE * 2.0,
                .. Default::default()
            },
            .. Default::default()
        });
}

fn spawn_player_hp(
    mut commands: Commands,
    hp_bg_query: Query<Entity, With<HpBackground>>,
    hp_bar_query: Query<Entity, With<HpBar>>,
    sprites: Res<Sprites>,
) {
    for ent in hp_bg_query.iter() {
        commands.entity(ent).despawn();
    }

    for ent in hp_bar_query.iter() {
        commands.entity(ent).despawn();
    }

    commands.spawn_bundle(SpriteBundle {
        material: sprites.health_background.clone(),
        transform: Transform {
            translation: Vec3::new(163., -112., 3.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(HpBackground);

    commands.spawn_bundle(SpriteBundle {
        material: sprites.health_bar.clone(),
        sprite: Sprite::new(Vec2::new(1.0, 1.0)),
        transform: health_bar_transform(&player_start_health()),
        .. Default::default()
    }).insert(HpBar);
}

fn player_start_health() -> Health {
    Health {
        current: START_HP,
        max: START_HP,
        vulnerable_to: vec![DamageType::Arrow, DamageType::Magic, DamageType::Ray,
                            DamageType::Sword],
    }
}

fn player_attack_visuals(
    mut commands: Commands,
    mut attack_reader: EventReader<PlayerAttackAction>,
    mut anim_query: Query<&mut AnimationState, With<Player>>,
    sprites: Res<Sprites>,
    time: Res<Time>,
) {
    if let Some(attack) = attack_reader.iter().next() {
        for mut anim in anim_query.single_mut() {
            *anim = AnimationState::Attacking {
                until: time.time_since_startup() + Duration::from_millis(300),
                damage_type: attack.damage_type.clone(),
            };

            if attack.damage_type == DamageType::Magic {
                commands.spawn()
                    .insert(DespawnAfter {
                        after: time.time_since_startup() + Duration::from_millis(300),
                    })
                    .insert_bundle(SpriteBundle {
                        material: sprites.magic_ball.clone(),
                        transform: Transform {
                            translation: Vec3::new(163., 23., 2.),
                            scale: Vec3::ONE * 0.25,
                            .. Default::default()
                        },
                        .. Default::default()
                    });
            }
        }
    }
}

fn player_defend_visuals(
    mut commands: Commands,
    mut defend_reader: EventReader<PlayerDefendAction>,
    sprites: Res<Sprites>,
    atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    if let Some(_defend) = defend_reader.iter().next() {
        commands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                .. Default::default()
            },
            texture_atlas: sprites.shield_flash.clone(),
            transform: Transform {
                translation: Vec3::new(163., 30., 2.),
                scale: Vec3::ONE * 0.2,
                .. Default::default()
            },
            .. Default::default()
        }).insert(DespawnAfter {
            after: time.time_since_startup() + Duration::from_millis(450),
        }).insert(AnimateSpriteSheet {
            frame_duration: Duration::from_millis(150),
            next_frame_time: time.time_since_startup() + Duration::from_millis(150),
            max_index: atlases.get(sprites.shield_flash.clone())
                              .map(|a| a.len() - 1).unwrap_or(0) as u32,
            loop_: false,
        });
    }
}

fn die(
    mut player: Query<(&mut AnimationState, &Health), With<Player>>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    time: Res<Time>,
) {
    for (mut anim, health) in player.single_mut() {
        let dead = health.current == 0;
        if dead {
            match *anim {
                AnimationState::Dead {..} => {},
                _ => {
                    // Just died.
                    *anim = AnimationState::Dead {
                        until: time.time_since_startup() + Duration::from_secs(2),
                    };
                    audio.play(sounds.scream.clone());
                }
            }
        }
    }
}

/// Update visuals from AnimationState
fn update_player_display(
    mut player: Query<(&mut AnimationState, &Health, &mut Handle<ColorMaterial>), With<Player>>,
    mut hp_display: Query<&mut Transform, With<HpBar>>,
    mut state: ResMut<State<GameState>>,
    sprites: Res<Sprites>,
    time: Res<Time>,
) {
    for (mut anim, health, mut mat) in player.single_mut() {
        match *anim {
            AnimationState::Dead {
                until,
            } => {
                *mat = sprites.dead.clone();
                if time.time_since_startup() > until {
                    state.set(GameState::GameOver).unwrap();
                }
            }
            AnimationState::Attacking {
                until, damage_type: ref dt,
            } if time.time_since_startup() < until => {
                *mat = match dt {
                    DamageType::Arrow => sprites.attack_arrow.clone(),
                    DamageType::Sword => sprites.attack_sword.clone(),
                    DamageType::Magic => sprites.attack_magic.clone(),
                    DamageType::Ray => unreachable!(),
                }
            },
            _ => {
                *mat = sprites.idle.clone();
                *anim = AnimationState::Idle;
            },
        }

        for mut hp_transform in hp_display.single_mut() {
            *hp_transform = health_bar_transform(&health);
        }
    }
}

/// TODO: This is O(n) in number of enemies, seems inefficient.
fn player_damage_applied(
    mut commands: Commands,
    mut damage_applied_reader: EventReader<DamageApplied>,
    player: Query<Entity, With<Player>>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    sprites: Res<Sprites>,
    time: Res<Time>,
) {
    let player_entity = player.single().unwrap();

    for damage_applied in damage_applied_reader.iter() {
        if damage_applied.damage.target == player_entity {
            audio.play(sounds.grunt.clone());

            // Spawn blood splatter
            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    .. Default::default()
                },
                texture_atlas: sprites.blood_splatter.clone(),
                transform: Transform {
                    translation: Vec3::new(163., -27., 3.),
                    scale: Vec3::ONE,
                    .. Default::default()
                },
                .. Default::default()
            }).insert(DespawnAfter {
                after: time.time_since_startup() + Duration::from_millis(800),
            }).insert(AnimateSpriteSheet {
                frame_duration: Duration::from_millis(100),
                next_frame_time: time.time_since_startup() + Duration::from_millis(100),
                max_index: 5,
                loop_: false,
            });
        }
    }
}

fn health_bar_transform(health: &Health) -> Transform {
    assert!(health.max >= 1);
    let portion = (health.current as f32) / (health.max as f32);
    let width_pixels = portion * 162.;

    Transform {
        translation: Vec3::new(163. - (162. / 2.) + width_pixels / 2., -112., 4.),
        scale: Vec3::new(width_pixels, 16., 1.),
        .. Default::default()
    }
}
