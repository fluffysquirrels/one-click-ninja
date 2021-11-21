use bevy::prelude::*;
use log::{debug, info};
use std::f32::consts::PI;

const WIN_W: f32 = 800.;
const WIN_H: f32 = 600.;

enum Action {
    Attack,
    Defend,
}

struct ActionIcon {
    action: Action,
}

struct ActionPointer {
    /// Angle of the pointer in radians
    angle: f32,

    /// Change of angle per second
    speed: f32,
}

struct ButtonPressed;
struct AttackAction;
struct DefendAction;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp_micros()
        .init();

    App::build()
        .insert_resource(WindowDescriptor {
            title: "One-Click Ninja".to_string(),
            width: WIN_W,
            height: WIN_H,
            .. Default::default()
        })
        .add_event::<ButtonPressed>()
        .add_event::<AttackAction>()
        .add_event::<DefendAction>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_action_spinner.system()))
        .add_system(spin_action_pointer.system())
        .add_system(keyboard_input.system())
        .add_system(choose_action.system())
        .run();
}

fn setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_action_spinner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sword_tex = asset_server.load("img/sword.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(sword_tex.into()),
        transform: Transform {
            translation: Vec3::new(-200., 100., 0.),
            scale: Vec3::new(0.3, 0.3, 0.3),
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionIcon { action: Action::Attack });

    let shield_tex = asset_server.load("img/shield.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(shield_tex.into()),
        transform: Transform {
            translation: Vec3::new(-200., -100., 0.),
            scale: Vec3::new(0.3, 0.3, 0.3),
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionIcon { action: Action::Defend });

    let pointer_tex = asset_server.load("img/pointer.png");
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
        speed: -PI,
    });
}

fn spin_action_pointer(
    time: Res<Time>,
    mut pointer_pos: Query<(&mut ActionPointer, &mut Transform)>
) {
    for (mut ap, mut transform) in pointer_pos.single_mut() {
        ap.angle = (ap.angle + time.delta_seconds() * ap.speed).rem_euclid(2. * PI);
        transform.rotation = Quat::from_rotation_z(ap.angle);
        trace!("spin_action_pointer: angle deg={}", ap.angle*180./PI);
    }
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
    mut attack_writer: EventWriter<AttackAction>,
    mut defend_writer: EventWriter<DefendAction>,
    pointer: Query<&ActionPointer>,
) {
    if button_reader.iter().next().is_some() {
        // Button was pressed, calculate the action.

        let ptr = pointer.single().unwrap();
        let deg = ptr.angle * 180. / PI;
        if deg >= 0. && deg <= 20. || deg >= 340. {
            debug!("choose_action: emit AttackAction");
            attack_writer.send(AttackAction);
        } else if deg >= 160. && deg <= 200. {
            debug!("choose_action: emit DefendAction");
            defend_writer.send(DefendAction);
        } else {
            // Missed all action icons, do nothing.
        }
    }
}
