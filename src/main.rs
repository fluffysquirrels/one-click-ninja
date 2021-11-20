use bevy::prelude::*;
use log::{debug, info};

const WIN_W: f32 = 800.;
const WIN_H: f32 = 600.;

enum ActionIcon {
    Attack,
    Defend,
}

struct ActionPointer {
    /// Angle of the pointer in radians
    angle: f32,

    /// Change of angle per second
    speed: f32,
}

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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_action_spinner.system()))
        .add_system(spin_action_pointer.system())
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
    let sword_tex = asset_server.load("sword.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(sword_tex.into()),
        transform: Transform {
            translation: Vec3::new(-200., 100., 0.),
            scale: Vec3::new(0.3, 0.3, 0.3),
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionIcon::Attack);

    let shield_tex = asset_server.load("shield.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(shield_tex.into()),
        transform: Transform {
            translation: Vec3::new(-200., -100., 0.),
            scale: Vec3::new(0.3, 0.3, 0.3),
            .. Default::default()
        },
        .. Default::default()
    }).insert(ActionIcon::Defend);

    let pointer_tex = asset_server.load("pointer.png");
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
        speed: -std::f32::consts::PI,
    });
}

fn spin_action_pointer(
    time: Res<Time>,
    mut pointer_pos: Query<(&mut ActionPointer, &mut Transform)>
) {
    for (mut ap, mut transform) in pointer_pos.single_mut() {
        ap.angle += time.delta_seconds() * ap.speed;
        transform.rotation = Quat::from_rotation_z(ap.angle);
    }
}
