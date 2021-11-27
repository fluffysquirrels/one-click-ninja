mod action_spinner;
mod components;
mod enemy;
mod events;
mod fight_display;
mod game_over;
mod game_state;
mod loading;
mod music;
mod player;
mod resources;
mod systems;
mod types;

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use crate::{
    resources::{Fonts, Icons, Sounds},
    game_state::GameState,
};

#[cfg(feature = "diagnostics")]
use {
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

struct Background {
    screen: Handle<ColorMaterial>,
    platform: Handle<ColorMaterial>,
}

const WIN_W: f32 = 800.;
const WIN_H: f32 = 600.;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp_micros()
        .init();

    log::info!("main()");

    let mut app = App::build();

    app
        .insert_resource(WindowDescriptor {
            title: "One-Click Ninja".to_string(),
            width: WIN_W,
            height: WIN_H,
            vsync: true, //Doesn't actually work (at least on linux)
            .. Default::default()
        })
        .add_event::<events::Damage>()
        .add_event::<events::DamageApplied>()
        .add_event::<events::Die>()
        .add_event::<events::EnemyAttackTime>()
        .add_event::<events::MusicTime>()
        .add_event::<events::PlayerAttackAction>()
        .add_event::<events::PlayerDefendAction>()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Loading)
        .add_plugin(AudioPlugin)
        .add_plugin(loading::Plugin)
        .add_plugin(action_spinner::Plugin)
        .add_plugin(enemy::Plugin)
        .add_plugin(fight_display::Plugin)
        .add_plugin(game_over::Plugin)
        .add_plugin(music::Plugin)
        .add_plugin(player::Plugin)
        .add_plugin(systems::animate_sprite_sheet::Plugin)
        .add_plugin(systems::damage::Plugin)
        .add_plugin(systems::despawn_after::Plugin)
        .add_plugin(systems::setup::Plugin)
        .add_plugin(systems::menu::Plugin)
        .add_system_set(
            SystemSet::on_enter(GameState::Menu)
                .with_system(setup.system()))
        .add_system_set(
            SystemSet::on_enter(GameState::CreateResources)
                .with_system(create_resources.system()))
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(on_enter_playing.system()))
        ;

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

fn create_resources(
    mut commands: Commands,
    font_assets: Res<loading::FontAssets>,
    audio_assets: Res<loading::AudioAssets>,
    texture_assets: Res<loading::TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: Should probably just pass audio_assets around.
    commands.insert_resource(Sounds {
        snare: audio_assets.snare.clone(),
        bass: audio_assets.bass.clone(),
        game_over: audio_assets.game_over.clone(),
        game_over_loop: audio_assets.game_over_loop.clone(),
        grunt: audio_assets.grunt.clone(),
        hit: audio_assets.hit.clone(),
        shield: audio_assets.shield.clone(),
        scream: audio_assets.scream.clone(),
        zombie_death: audio_assets.zombie_death.clone(),
    });

    commands.insert_resource(Icons {
        attack: materials.add(texture_assets.icon_sword.clone().into()),
        defend: materials.add(texture_assets.icon_shield.clone().into()),
    });

    // TODO: Should probably just pass audio_assets around.
    commands.insert_resource(Fonts {
        default: font_assets.fira_sans.clone(),
    });

    commands.insert_resource(Background {
        screen: materials.add(texture_assets.background.clone().into()),
        platform: materials.add(texture_assets.platform.clone().into()),
    });
}

fn setup(
    mut commands: Commands,
    background: Res<Background>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        material: background.screen.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            .. Default::default()
        },
        .. Default::default()
    });
}

fn on_enter_playing(
    mut commands: Commands,
    background: Res<Background>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: background.platform.clone(),
        transform: Transform {
            translation: Vec3::new(163., -7., 1.),
            scale: Vec3::ONE * 1.5,
            .. Default::default()
        },
        .. Default::default()
    });
}
