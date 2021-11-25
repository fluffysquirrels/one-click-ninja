use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    components::{Action, Health, Player},
    events::{EnemyAttackTime, PlayerAttackAction, PlayerDefendAction},
    Sounds,
    types::{DamageType},
};
use std::f32::consts::PI;

struct ActionIcon {
    #[allow(dead_code)] // WIP.
    action: Action,
}

struct ActionPointer {
    /// Angle of the pointer in radians
    angle: f32,

    /// Change of angle per second
    speed: f32,
}

struct ButtonPressed;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ButtonPressed>()
           .add_startup_system(spawn_action_spinner.system())
           .add_system(spin_action_pointer.system())
           .add_system(keyboard_input.system())
           .add_system(choose_action.system());
    }
}

fn spawn_action_spinner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sword_tex = asset_server.load("sprites/sword.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(sword_tex.into()),
        transform: Transform {
            translation: Vec3::new(-200., 100., 0.),
            scale: Vec3::ONE * 0.3,
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionIcon { action: Action::AttackSword });

    let shield_tex = asset_server.load("sprites/shield.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(shield_tex.into()),
        transform: Transform {
            translation: Vec3::new(-200., -100., 0.),
            scale: Vec3::ONE * 0.3,
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionIcon { action: Action::Defend });

    let magic_tex = asset_server.load("sprites/magic_ball.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(magic_tex.into()),
        transform: Transform {
            translation: Vec3::new(-300., 0., 0.),
            scale: Vec3::ONE,
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionIcon { action: Action::AttackMagic });

    let arrow_tex = asset_server.load("sprites/arrow.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(arrow_tex.into()),
        transform: Transform {
            translation: Vec3::new(-100., 0., 0.),
            scale: Vec3::ONE * 0.5,
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionIcon { action: Action::AttackArrow });

    let pointer_tex = asset_server.load("sprites/pointer.png");
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(5., 40.)),
        material: materials.add(pointer_tex.into()),
        transform: Transform {
            translation: Vec3::new(-200., 0., 1.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionPointer {
        angle: 0.,
        speed: -PI * 80. / 60., // 80 bpm
    });
}

const ATTACK_SWORD_ANGLE: f32 = 0.;
const ATTACK_ARROW_ANGLE: f32 = PI * 1.5;
const ATTACK_MAGIC_ANGLE: f32 = PI * 0.5;
const DEFEND_ANGLE: f32 = PI;
const ENEMY_ATTACK_ANGLE: f32 = 160. * PI / 180.;

fn spin_action_pointer(
    time: Res<Time>,
    mut pointer_pos: Query<(&mut ActionPointer, &mut Transform)>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    mut enemy_attack_time_writer: EventWriter<EnemyAttackTime>,
) {
    for (mut ap, mut transform) in pointer_pos.single_mut() {
        let old_angle = ap.angle;
        let new_angle = old_angle + time.delta_seconds() * ap.speed;
        if is_angle_hit(old_angle, new_angle, ATTACK_SWORD_ANGLE) {
            audio.play(sounds.snare.clone());
        }

        if is_angle_hit(old_angle, new_angle, ATTACK_ARROW_ANGLE) {
            audio.play(sounds.snare.clone());
        }

        if is_angle_hit(old_angle, new_angle, ATTACK_MAGIC_ANGLE) {
            audio.play(sounds.snare.clone());
        }

        if is_angle_hit(old_angle, new_angle, DEFEND_ANGLE) {
            trace!("play bass");
            audio.play(sounds.bass.clone());
        }

        if is_angle_hit(old_angle, new_angle, ENEMY_ATTACK_ANGLE) {
            enemy_attack_time_writer.send(EnemyAttackTime);
        }

        ap.angle = new_angle.rem_euclid(2. * PI);
        transform.rotation = Quat::from_rotation_z(ap.angle);
        trace!("spin_action_pointer: angle deg={}", ap.angle*180./PI);
    }
}

fn is_angle_hit(old_angle: f32, new_angle: f32, target_angle: f32) -> bool {
    (old_angle - target_angle).signum() != (new_angle - target_angle).signum()
}

fn keyboard_input(
    // Maybe debounce with Time?
    // time: Res<Time>,

    kb: Res<Input<KeyCode>>,
    mut button_writer: EventWriter<ButtonPressed>,
) {
    if kb.just_pressed(KeyCode::Space) {
        debug!("keyboard_input: emit ButtonPressed");
        button_writer.send(ButtonPressed);
    }
}

fn choose_action(
    mut button_reader: EventReader<ButtonPressed>,
    mut attack_writer: EventWriter<PlayerAttackAction>,
    mut defend_writer: EventWriter<PlayerDefendAction>,
    pointer: Query<&ActionPointer>,
    player: Query<&Health, With<Player>>
) {
    if button_reader.iter().next().is_some() {
        // Button was pressed

        match player.single() {
            Ok(health) if health.current > 0 => {
                let ptr = pointer.single().unwrap();
                let deg = ptr.angle * 180. / PI;
                if deg >= 0. && deg <= 20. || deg >= 340. {
                    let attack = PlayerAttackAction {
                        damage_type: DamageType::Sword,
                    };
                    debug!("choose_action: emit {:?}", attack);
                    attack_writer.send(attack);
                } else if deg >= 70. && deg <= 110. {
                    let attack = PlayerAttackAction {
                        damage_type: DamageType::Magic,
                    };
                    debug!("choose_action: emit {:?}", attack);
                    attack_writer.send(attack);
                } else if deg >= 250. && deg <= 290. {
                    let attack = PlayerAttackAction {
                        damage_type: DamageType::Arrow,
                    };
                    debug!("choose_action: emit {:?}", attack);
                    attack_writer.send(attack);
                } else if deg >= 160. && deg <= 200. {
                    debug!("choose_action: emit PlayerDefendAction");
                    defend_writer.send(PlayerDefendAction);
                } else {
                    // Missed all action icons, do nothing.
                }
            },
            _ => {},
        };
    }
}
