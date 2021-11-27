use bevy::prelude::*;
// use bevy_kira_audio::Audio;

use crate::{
    components::{Action, Health, Player},
    events::{EnemyAttackTime, MusicTime, PlayerAttackAction, PlayerDefendAction},
    // Sounds,
    types::{DamageType},
    game_state::GameState,
    loading,
};
use std::f64::consts::PI;

struct ActionIcon {
    action: Action,
}

struct ActionPointer {
    /// Angle of the pointer in radians
    angle: f64,
}

struct ButtonPressed;

struct ActionSpinner;

struct PlayerAttackedThisTurn(bool);

struct Icons {
    pointer: Handle<ColorMaterial>,
    sword: Handle<ColorMaterial>,
    shield: Handle<ColorMaterial>,
    magic: Handle<ColorMaterial>,
    arrow: Handle<ColorMaterial>,
    sword_highlight: Handle<ColorMaterial>,
    shield_highlight: Handle<ColorMaterial>,
    magic_highlight: Handle<ColorMaterial>,
    arrow_highlight: Handle<ColorMaterial>,
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ButtonPressed>()
            .add_system_set(
                SystemSet::on_enter(GameState::CreateResources)
                    .with_system(create_resources.system()))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_entities.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                   .with_system(spin_action_pointer.system())
                   .with_system(keyboard_input.system())
                   .with_system(choose_action.system())
            );
    }
}

fn create_resources(
    mut commands: Commands,
    texture_assets: Res<loading::TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Icons {
        pointer: materials.add(texture_assets.icon_pointer.clone().into()),
        sword: materials.add(texture_assets.icon_sword.clone().into()),
        shield: materials.add(texture_assets.icon_shield.clone().into()),
        magic: materials.add(texture_assets.icon_magic.clone().into()),
        arrow: materials.add(texture_assets.icon_arrow.clone().into()),
        sword_highlight: materials.add(texture_assets.icon_sword_highlight.clone().into()),
        shield_highlight: materials.add(texture_assets.icon_shield_highlight.clone().into()),
        magic_highlight: materials.add(texture_assets.icon_magic_highlight.clone().into()),
        arrow_highlight: materials.add(texture_assets.icon_arrow_highlight.clone().into()),
    });
}

fn spawn_entities(
    mut commands: Commands,
    existing_query: Query<Entity, With<ActionSpinner>>,
    icons: Res<Icons>,
) {
    for ent in existing_query.iter() {
        commands.entity(ent).despawn();
    }

    commands.insert_resource(PlayerAttackedThisTurn(false));

    commands.spawn_bundle(SpriteBundle {
        material: icons.sword.clone(),
        transform: Transform {
            translation: Vec3::new(-200., 100., 0.),
            scale: Vec3::ONE * 0.3,
            .. Default::default()
        },
        .. Default::default()
    })
        .insert(ActionIcon { action: Action::AttackSword })
        .insert(ActionSpinner);

    commands.spawn_bundle(SpriteBundle {
        material: icons.shield.clone(),
        transform: Transform {
            translation: Vec3::new(-200., -100., 0.),
            scale: Vec3::ONE * 0.3,
            .. Default::default()
        },
        .. Default::default()
    })
        .insert(ActionIcon { action: Action::Defend })
        .insert(ActionSpinner);

    commands.spawn_bundle(SpriteBundle {
        material: icons.magic.clone(),
        transform: Transform {
            translation: Vec3::new(-300., 0., 0.),
            scale: Vec3::ONE * 0.4,
            .. Default::default()
        },
        .. Default::default()
    })
        .insert(ActionIcon { action: Action::AttackMagic })
        .insert(ActionSpinner);

    commands.spawn_bundle(SpriteBundle {
        material: icons.arrow.clone(),
        transform: Transform {
            translation: Vec3::new(-100., 0., 0.),
            scale: Vec3::ONE * 0.5,
            .. Default::default()
        },
        .. Default::default()
    })
        .insert(ActionIcon { action: Action::AttackArrow })
        .insert(ActionSpinner);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(5., 40.)),
        material: icons.pointer.clone(),
        transform: Transform {
            translation: Vec3::new(-200., 0., 1.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionPointer {
        angle: 0.,
    }).insert(ActionSpinner);
}

const ATTACK_SWORD_ANGLE: f64 = 0.;
const ATTACK_ARROW_ANGLE: f64 = PI * 1.5;
const ATTACK_MAGIC_ANGLE: f64 = PI * 0.5;
const DEFEND_ANGLE: f64 = PI;
const ENEMY_ATTACK_ANGLE: f64 = 160. * PI / 180.;

fn spin_action_pointer(
    mut music_time_reader: EventReader<MusicTime>,
    mut enemy_attack_time_writer: EventWriter<EnemyAttackTime>,
    mut pointer_pos: Query<(&mut ActionPointer, &mut Transform)>,
    mut attacked_this_turn: ResMut<PlayerAttackedThisTurn>,
) {
    for (mut ap, mut transform) in pointer_pos.single_mut() {

        // TODO
        let icons: Vec<ActionIcon> = vec![];

        let music_time = music_time_reader.iter().last();
        let old_angle = ap.angle;
        let new_angle =
            music_time.map(|mt| (PI - mt.beat_in_bar * (1./4.) * 2. * PI).rem_euclid(2. * PI))
                      .unwrap_or(old_angle);

        if in_angle_range(new_angle, ATTACK_SWORD_ANGLE) {
            highlight_icon(icons.iter().find(|icon| icon.action == Action::AttackSword));
        }

        if in_angle_range(new_angle, ATTACK_ARROW_ANGLE) {
            highlight_icon(icons.iter().find(|icon| icon.action == Action::AttackArrow));
        }

        if in_angle_range(new_angle, ATTACK_MAGIC_ANGLE) {
            highlight_icon(icons.iter().find(|icon| icon.action == Action::AttackMagic));
        }

        if in_angle_range(new_angle, DEFEND_ANGLE) {
            highlight_icon(icons.iter().find(|icon| icon.action == Action::Defend));
        }

        if is_angle_hit(old_angle, new_angle, DEFEND_ANGLE) {
            attacked_this_turn.0 = false;
        }

        if is_angle_hit(old_angle, new_angle, ENEMY_ATTACK_ANGLE) {
            enemy_attack_time_writer.send(EnemyAttackTime);
        }

        ap.angle = new_angle.rem_euclid(2. * PI);
        transform.rotation = Quat::from_rotation_z(ap.angle as f32);
        trace!("spin_action_pointer: angle deg={}", ap.angle*180./PI);
    }
}

fn in_angle_range(angle: f64, target_angle: f64) -> bool {
    // TODO
    false
}

fn highlight_icon(icon: Option<&ActionIcon>) {
    // TODO.
}

fn is_angle_hit(old_angle: f64, new_angle: f64, target_angle: f64) -> bool {
    let old_angle = if new_angle > old_angle {
        old_angle + 2. * PI
    } else {
        old_angle
    };
    (old_angle - target_angle).signum() !=
        (new_angle - target_angle).signum()
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
    player: Query<&Health, With<Player>>,
    mut attacked_this_turn: ResMut<PlayerAttackedThisTurn>,
) {
    if button_reader.iter().next().is_some() {
        // Button was pressed

        match player.single() {
            Ok(health) if health.current > 0 => {
                let ptr = pointer.single().unwrap();
                let deg = ptr.angle * 180. / PI;
                if deg >= 160. && deg <= 200. {
                    debug!("choose_action: emit PlayerDefendAction");
                    defend_writer.send(PlayerDefendAction);
                } else if !attacked_this_turn.0 {
                    attacked_this_turn.0 = true;
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
                    }
                }
            },
            _ => {},
        };
    }
}
