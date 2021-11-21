mod action_spinner;
mod components;
mod enemy;

use bevy::prelude::*;
use crate::components::{EnemyAttackTime, PlayerAttackAction, PlayerDefendAction};

struct Sounds {
    bass: Handle<AudioSource>,
    snare: Handle<AudioSource>,
}

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
            .. Default::default()
        })
        .add_event::<PlayerAttackAction>()
        .add_event::<PlayerDefendAction>()
        .add_event::<EnemyAttackTime>()
        .add_plugins(DefaultPlugins)
        .add_plugin(action_spinner::Plugin)
        .add_plugin(enemy::Plugin)
        .add_startup_system(setup.system())
        .add_startup_system(load_resources.system().label("load"));


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
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Sounds {
        snare: asset_server.load("sfx/kenney_uiaudio/Audio/click1.ogg"),
        bass: asset_server.load("sfx/kenney_uiaudio/Audio/click2.ogg"),
    });
}
