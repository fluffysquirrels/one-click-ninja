use bevy::prelude::*;
use crate::{
    components::{Health, Player},
    types::Hp,
};

pub struct Plugin;

struct PlayerHpDisplay;

const START_HP: Hp = 10;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_player_hp.system())
            .add_startup_system(spawn_player.system())
            .add_system(update_player_hp.system());
    }
}

fn spawn_player(
    mut commands: Commands,
) {
    commands
        .spawn()
        .insert(Player)
        .insert(Health {
            current: START_HP,
            max: START_HP,
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
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::RED,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        .. Default::default()
    }).insert(PlayerHpDisplay);
}

fn update_player_hp(
    player: Query<(&Player, &Health)>,
    mut display: Query<(&PlayerHpDisplay, &mut Text)>,
) {
    for (_, health) in player.single() {
        for (_, mut text) in display.single_mut() {
            text.sections[0].value = format_hp(health.current, health.max);
        }
    }
}

fn format_hp(curr_hp: Hp, max_hp: Hp) -> String {
    format!("Player HP = {}/{}", curr_hp, max_hp)
}
