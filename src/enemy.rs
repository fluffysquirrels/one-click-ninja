use bevy::prelude::*;
use crate::components::{Enemy, EnemyAttackTime};
use std::time::Duration;

pub struct Plugin;

struct Sprites {
    idle: Handle<ColorMaterial>,
    attack: Handle<ColorMaterial>,
}

struct AttackAnimation {
    until: std::time::Duration,
}

const LOAD: &'static str = "load";

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(load_resources.system().label(LOAD))
            .add_startup_system(spawn_current_enemy.system().after(LOAD))
            .add_system(enemy_attack.system())
            .add_system(attack_animation.system());
    }
}

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Sprites {
        idle: materials.add(
            asset_server.load("sprites/Samurai02/01-Idle/01-Normal/2D_SM02_Idle_000.png").into()),
        attack: materials.add(
            asset_server.load("sprites/Samurai02/03-Attack/2D_SM02_Attack_004.png").into()),
    });
}

/// This should use the sprite loaded in load_resources, but even with the
/// system ordering using labels, the Sprites resource is not available when
/// this startup system runs.
fn spawn_current_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // sprites: Res<Sprites>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(
            asset_server.load("sprites/Samurai02/01-Idle/01-Normal/2D_SM02_Idle_000.png").into()),
        transform: Transform {
            translation: Vec3::new(200., 200., 0.),
            scale: Vec3::new(0.3, 0.3, 0.3),
            .. Default::default()
        },
        .. Default::default()
    }).insert(Enemy);
}

fn enemy_attack(
    mut commands: Commands,
    mut enemy: Query<(Entity, &Enemy, &mut Handle<ColorMaterial>)>,
    mut attack_time_reader: EventReader<EnemyAttackTime>,
    sprites: Res<Sprites>,
    time: Res<Time>,
) {
    for (entity, _enemy, mut material) in enemy.single_mut() {
        if attack_time_reader.iter().next().is_some() {
            commands.entity(entity).insert(AttackAnimation {
                until: time.time_since_startup() + Duration::from_millis(300)
            });
            *material = sprites.attack.clone();
        }
    }
}

fn attack_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy: Query<(Entity, &Enemy, &mut Handle<ColorMaterial>, &AttackAnimation)>,
    sprites: Res<Sprites>,
) {
    for (entity, _enemy, mut material, anim) in enemy.single_mut() {
        if time.time_since_startup() > anim.until {
            *material = sprites.idle.clone();
            commands.entity(entity).remove::<AttackAnimation>();
        }
    }
}
