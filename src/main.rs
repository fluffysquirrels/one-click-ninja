mod action_spinner;
mod components;
mod enemy;
mod events;
mod fight_display;
mod player;
mod resources;
mod types;

use bevy::prelude::*;
use crate::{
    components::Health,
    events::{Damage, Die, EnemyAttackTime, PlayerAttackAction, PlayerDefendAction},
    resources::{Icons, Sounds},
};

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
            // vsync: true,
            .. Default::default()
        })
        .add_event::<Damage>()
        .add_event::<Die>()
        .add_event::<EnemyAttackTime>()
        .add_event::<PlayerAttackAction>()
        .add_event::<PlayerDefendAction>()
        .add_plugins(DefaultPlugins)
        .add_plugin(action_spinner::Plugin)
        .add_plugin(enemy::Plugin)
        .add_plugin(fight_display::Plugin)
        .add_plugin(player::Plugin)
        .add_startup_system(setup.system())
        .add_startup_system(load_resources.system().label("load"))
        .add_system(process_damage.system());


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
        attack: materials.add(asset_server.load("img/sword.png").into()),
        defend: materials.add(asset_server.load("img/shield.png").into()),
    });
}

fn process_damage(
    mut damage_reader: EventReader<Damage>,
    mut die_writer: EventWriter<Die>,
    mut health_query: Query<&mut Health>,
) {
    for damage in damage_reader.iter() {
        let mut health = match health_query.get_mut(damage.target) {
            Err(e) => {
                error!("No Health component for Damage.target entity; error: {}", e);
                continue;
            },
            Ok(h) => h,
        };
        health.current = health.current.checked_sub(damage.hp).unwrap_or(0);
        if health.current == 0 {
            die_writer.send(Die {
                target: damage.target,
            });
        }
    }
}
