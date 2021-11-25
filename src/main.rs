mod action_spinner;
mod components;
mod enemy;
mod events;
mod fight_display;
mod player;
mod resources;
mod types;

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use crate::{
    components::{DespawnAfter, Health},
    events::{Damage, Die, EnemyAttackTime, PlayerAttackAction, PlayerDefendAction},
    resources::{Fonts, Icons, Sounds},
};
use std::time::Duration;

#[cfg(feature = "diagnostics")]
use {
    std::time::Duration,

    bevy::{
        diagnostic::{
            EntityCountDiagnosticsPlugin,
            LogDiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
        },
        asset::diagnostic::AssetCountDiagnosticsPlugin,
    },
};

#[cfg(all(feature = "native", feature = "diagnostics"))]
use bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin;


const WIN_W: f32 = 800.;
const WIN_H: f32 = 600.;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp_micros()
        .init();

    let mut app = App::build();

    app
        .insert_resource(WindowDescriptor {
            title: "One-Click Ninja".to_string(),
            width: WIN_W,
            height: WIN_H,
            vsync: true, //Doesn't actually work (at least on linux)
            .. Default::default()
        })
        .add_event::<Damage>()
        .add_event::<Die>()
        .add_event::<EnemyAttackTime>()
        .add_event::<PlayerAttackAction>()
        .add_event::<PlayerDefendAction>()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)

        .add_plugin(action_spinner::Plugin)
        .add_plugin(enemy::Plugin)
        .add_plugin(fight_display::Plugin)
        .add_plugin(player::Plugin)
        .add_startup_system(setup.system())
        .add_startup_system_to_stage(StartupStage::PreStartup, load_resources.system())
        .add_system(process_damage.system())
        .add_system(despawn_after.system());


    #[cfg(feature = "diagnostics")]
    {
        app
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(EntityCountDiagnosticsPlugin::default())
            .add_plugin(AssetCountDiagnosticsPlugin::<Texture>::default());

        #[cfg(feature = "native")]
        app.add_plugin(WgpuResourceDiagnosticsPlugin::default());
    }

    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Sounds {
        snare: asset_server.load("sfx/kenney_uiaudio/Audio/click1.ogg"),
        bass: asset_server.load("sfx/kenney_uiaudio/Audio/click2.ogg"),
    });

    commands.insert_resource(Icons {
        attack: materials.add(asset_server.load("sprites/sword.png").into()),
        defend: materials.add(asset_server.load("sprites/shield.png").into()),
    });

    commands.insert_resource(Fonts {
        default: asset_server.load("fonts/FiraSans-Bold.ttf"),
    });
}

struct DamageDisplay;

fn process_damage(
    mut commands: Commands,
    mut damage_reader: EventReader<Damage>,
    mut die_writer: EventWriter<Die>,
    mut health_query: Query<(&mut Health, &Transform)>,
    fonts: Res<Fonts>,
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

            commands.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    "Hit!",
                    TextStyle {
                        font: fonts.default.clone(),
                        font_size: 30.0,
                        color: Color::RED,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                    ),
                transform: Transform {
                    translation: health_transform.translation + Vec3::new(100., 0., 0.),
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
            commands.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    "Invulnerable!",
                    TextStyle {
                        font: fonts.default.clone(),
                        font_size: 30.0,
                        color: Color::RED,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                    ),
                transform: Transform {
                    translation: health_transform.translation + Vec3::new(100., 0., 0.),
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

fn despawn_after(
    mut commands: Commands,
    query: Query<(Entity, &DespawnAfter)>,
    time: Res<Time>,
) {
    let now = time.time_since_startup();
    for (entity, despawn_after) in query.iter() {
        if now > despawn_after.after {
            commands.entity(entity).despawn();
        }
    }
}
