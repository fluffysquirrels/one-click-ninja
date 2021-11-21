mod action_spinner;
mod enemy;

use bevy::prelude::*;

enum Action {
    Attack,
    Defend,
}

struct AttackAction;
struct DefendAction;

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

    App::build()
        .insert_resource(WindowDescriptor {
            title: "One-Click Ninja".to_string(),
            width: WIN_W,
            height: WIN_H,
            .. Default::default()
        })
        .add_event::<AttackAction>()
        .add_event::<DefendAction>()
        .add_plugins(DefaultPlugins)
        .add_plugin(action_spinner::Plugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Sounds {
        snare: asset_server.load("sfx/kenney_uiaudio/Audio/click1.ogg"),
        bass: asset_server.load("sfx/kenney_uiaudio/Audio/click2.ogg"),
    });
}
