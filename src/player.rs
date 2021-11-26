use bevy::prelude::*;
use crate::{
    components::{DespawnAfter, Health, Player},
    events::PlayerAttackAction,
    resources::Fonts,
    types::{DamageType, Hp},
    game_state::GameState,
    loading,
};
use std::time::Duration;

pub struct Plugin;

struct PlayerHpDisplay;

const START_HP: Hp = 10;

struct Sprites {
    idle: Handle<ColorMaterial>,
    attack_arrow: Handle<ColorMaterial>,
    attack_magic: Handle<ColorMaterial>,
    attack_sword: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
    magic_ball: Handle<ColorMaterial>,
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
                    .with_system(player_attack.system())
                    .with_system(die.system()))
            ;
    }
}

fn create_resources(
    mut commands: Commands,
    texture_assets: Res<loading::TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Sprites {
        idle: materials.add(texture_assets.player_idle.clone().into()),
        attack_arrow: materials.add(texture_assets.player_attack_arrow.clone().into()),
        attack_sword: materials.add(texture_assets.player_attack_sword.clone().into()),
        attack_magic: materials.add(texture_assets.player_attack_magic.clone().into()),
        dead: materials.add(texture_assets.player_dead.clone().into()),
        magic_ball: materials.add(texture_assets.icon_magic.clone().into()),
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
        .insert(Health {
            current: START_HP,
            max: START_HP,
            vulnerable_to: vec![DamageType::Arrow, DamageType::Magic, DamageType::Sword],
        })
        .insert(AnimationState::Idle)
        .insert_bundle(SpriteBundle {
            material: sprites.idle.clone(),
            transform: Transform {
                translation: Vec3::new(100., 0., 2.),
                scale: Vec3::ONE * 2.0,
                .. Default::default()
            },
            .. Default::default()
        });
}

fn spawn_player_hp(
    mut commands: Commands,
    player_hp_query: Query<Entity, With<PlayerHpDisplay>>,
    fonts: Res<Fonts>,
) {
    for ent in player_hp_query.iter() {
        commands.entity(ent).despawn();
    }

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            format_hp(START_HP, START_HP),
            TextStyle {
                font: fonts.default.clone(),
                font_size: 30.0,
                color: Color::GREEN,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        transform: Transform {
            translation: Vec3::new(100., -85., 2.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(PlayerHpDisplay);
}

fn player_attack(
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
                            translation: Vec3::new(100., 50., 1.),
                            scale: Vec3::ONE * 0.25,
                            .. Default::default()
                        },
                        .. Default::default()
                    });
            }
        }
    }
}

fn die(
    mut player: Query<(&mut AnimationState, &Health), With<Player>>,
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
                    }
                }
            }
        }
    }
}

/// Update visuals from AnimationState
fn update_player_display(
    mut player: Query<(&mut AnimationState, &Health, &mut Handle<ColorMaterial>), With<Player>>,
    mut display_hp: Query<(&PlayerHpDisplay, &mut Text)>,
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
                }
            },
            _ => {
                *mat = sprites.idle.clone();
                *anim = AnimationState::Idle;
            },
        }

        for (_, mut text) in display_hp.single_mut() {
            text.sections[0].value = format_hp(health.current, health.max);
        }
    }
}

fn format_hp(curr_hp: Hp, max_hp: Hp) -> String {
    format!("Player HP = {}/{}", curr_hp, max_hp)
}
