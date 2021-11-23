use bevy::prelude::*;
use crate::{
    components::{Health, Player},
    types::Hp,
};

pub struct Plugin;

struct PlayerHpDisplay;

const START_HP: Hp = 3;

struct Sprites {
    idle: Handle<ColorMaterial>,
    attack: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_resources.system())
            .add_startup_system(spawn_player_hp.system())
            .add_startup_system(spawn_player.system())
            .add_system(update_player_hp.system());
    }
}

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Sprites {
        idle: materials.add(
            asset_server.load("sprites/lpc-medieval-fantasy-character/our_work/player/walk_up/00.png").into()),
        attack: materials.add(
            asset_server.load("sprites/lpc-medieval-fantasy-character/our_work/player/spear_up/05.png").into()),        dead: materials.add(
            asset_server.load("sprites/lpc-medieval-fantasy-character/our_work/player/die/05.png").into()),
    });
}

fn spawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
    commands
        .spawn()
        .insert(Player)
        .insert(Health {
            current: START_HP,
            max: START_HP,
        })
        .insert_bundle(SpriteBundle {
            material: sprites.idle.clone(),
            transform: Transform {
                translation: Vec3::new(200., 0., 0.),
                scale: Vec3::ONE * 2.0,
                .. Default::default()
            },
            .. Default::default()
        });
}

fn spawn_player_hp(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            format_hp(START_HP, START_HP),
            TextStyle {
                // TODO: Stash this in a resource
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::GREEN,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        transform: Transform {
            translation: Vec3::new(200., -200., 0.),
            .. Default::default()
        },
        .. Default::default()
    }).insert(PlayerHpDisplay);
}

fn update_player_hp(
    mut player: Query<(&Player, &Health, &mut Handle<ColorMaterial>)>,
    mut display_hp: Query<(&PlayerHpDisplay, &mut Text)>,
    sprites: Res<Sprites>,
) {
    for (_, health, mut mat) in player.single_mut() {
        if health.current == 0 {
            *mat = sprites.dead.clone();
        }
        for (_, mut text) in display_hp.single_mut() {
            text.sections[0].value = format_hp(health.current, health.max);
        }
    }
}

fn format_hp(curr_hp: Hp, max_hp: Hp) -> String {
    format!("Player HP = {}/{}", curr_hp, max_hp)
}
