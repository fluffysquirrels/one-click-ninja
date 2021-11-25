use bevy::prelude::*;
use crate::{
    components::{Health, Player},
    events::PlayerAttackAction,
    resources::Fonts,
    types::{DamageType, Hp},
};
use std::time::Duration;

pub struct Plugin;

struct PlayerHpDisplay;

const START_HP: Hp = 5;

struct Sprites {
    idle: Handle<ColorMaterial>,
    attack_arrow: Handle<ColorMaterial>,
    attack_magic: Handle<ColorMaterial>,
    attack_sword: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
}

enum AnimationState {
    Dead,
    Attacking {
        until: Duration,
        damage_type: DamageType
    },
    Idle,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_resources.system())
            .add_startup_system(spawn_player_hp.system())
            .add_startup_system(spawn_player.system())
            .add_system_to_stage(CoreStage::PostUpdate, update_player_display.system())
            .add_system(player_attack.system());
    }
}

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Sprites {
        idle: materials.add(
            asset_server.load(
                "sprites/lpc-medieval-fantasy-character/our_work/player/walk_up/00.png").into()),
        attack_arrow: materials.add(
            asset_server.load(
                "sprites/lpc-medieval-fantasy-character/our_work/player/bow_up/09.png").into()),
        attack_sword: materials.add(
            asset_server.load(
                "sprites/lpc-medieval-fantasy-character/our_work/player/spear_up/05.png").into()),
        attack_magic: materials.add(
            asset_server.load(
                "sprites/lpc-medieval-fantasy-character/our_work/player/cast_up/05.png").into()),
        dead: materials.add(
            asset_server.load(
                "sprites/lpc-medieval-fantasy-character/our_work/player/die/05.png").into()),
    });
}

fn spawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
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
                translation: Vec3::new(100., 0., 0.),
                scale: Vec3::ONE * 2.0,
                .. Default::default()
            },
            .. Default::default()
        });
}

fn spawn_player_hp(
    mut commands: Commands,
    fonts: Res<Fonts>,
) {
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
            translation: Vec3::new(100., -100., 0.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(PlayerHpDisplay);
}

fn player_attack(
    mut attack_reader: EventReader<PlayerAttackAction>,
    mut anim_query: Query<&mut AnimationState, With<Player>>,
    time: Res<Time>,
) {
    if let Some(attack) = attack_reader.iter().next() {
        for mut anim in anim_query.single_mut() {
            *anim = AnimationState::Attacking {
                until: time.time_since_startup() + Duration::from_millis(300),
                damage_type: attack.damage_type.clone(),
            };
        }
    }
}

fn update_player_display(
    mut player: Query<(&mut AnimationState, &Health, &mut Handle<ColorMaterial>), With<Player>>,
    mut display_hp: Query<(&PlayerHpDisplay, &mut Text)>,
    sprites: Res<Sprites>,
    time: Res<Time>,
) {
    for (mut anim, health, mut mat) in player.single_mut() {
        let dead = health.current == 0;
        if dead {
            *mat = sprites.dead.clone();
            *anim = AnimationState::Dead;
        } else {
            match *anim {
                AnimationState::Dead => unreachable!(),
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
        }
        for (_, mut text) in display_hp.single_mut() {
            text.sections[0].value = format_hp(health.current, health.max);
        }
    }
}

fn format_hp(curr_hp: Hp, max_hp: Hp) -> String {
    format!("Player HP = {}/{}", curr_hp, max_hp)
}
