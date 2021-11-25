use bevy::prelude::*;
use crate::{
    components::{DespawnAfter, Health},
    events::{Die, Damage},
};
use std::time::Duration;

struct DamageSprites {
    hit: Handle<ColorMaterial>,
    blocked: Handle<ColorMaterial>,
}

struct DamageDisplay;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_resources.system())
            .add_system(process_damage.system());
    }
}

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(DamageSprites {
        hit: materials.add(asset_server.load("sprites/david_dawn/hit.png").into()),
        blocked: materials.add(asset_server.load("sprites/david_dawn/blocked.png").into()),
    });
}

fn process_damage(
    mut commands: Commands,
    mut damage_reader: EventReader<Damage>,
    mut die_writer: EventWriter<Die>,
    mut health_query: Query<(&mut Health, &Transform)>,
    sprites: Res<DamageSprites>,
    time: Res<Time>,
) {
    for damage in damage_reader.iter() {
        let (mut health, health_transform) = match health_query.get_mut(damage.target) {
            Err(e) => {
                error!("No Health component for Damage.target entity; error: {}", e);
                continue;
            },
            Ok(h) => h,
        };
        if health.vulnerable_to.contains(&damage.damage_type) {
            // Vulnerable to damage
            health.current = health.current.checked_sub(damage.hp).unwrap_or(0);
            if health.current == 0 {
                die_writer.send(Die {
                    target: damage.target,
                });
            }

            commands.spawn_bundle(SpriteBundle {
                material: sprites.hit.clone(),
                transform: Transform {
                    translation: health_transform.translation + Vec3::new(150., 0., 0.),
                    scale: Vec3::ONE * 0.75,
                    .. Default::default()
                },
                .. Default::default()
            })
                .insert(DamageDisplay)
                .insert(DespawnAfter {
                    after: time.time_since_startup() + Duration::from_millis(300),
                });
        } else {
            // Not vulnerable to damage.
            commands.spawn_bundle(SpriteBundle {
                material: sprites.blocked.clone(),
                transform: Transform {
                    translation: health_transform.translation + Vec3::new(150., 0., 0.),
                    scale: Vec3::ONE * 0.5,
                    .. Default::default()
                },
                .. Default::default()
            })
                .insert(DamageDisplay)
                .insert(DespawnAfter {
                    after: time.time_since_startup() + Duration::from_millis(300),
                });
        }
    }
}
