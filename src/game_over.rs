use bevy::prelude::*;
use bevy_kira_audio::Audio;
use crate::{
    game_state::GameState,
    loading,
    Sounds,
};

struct GameOver;

struct Sprites {
    press_space: Handle<ColorMaterial>,
    text: Handle<ColorMaterial>,
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Setup)
                    .with_system(create_resources.system()))
            .add_system_set(
                SystemSet::on_enter(GameState::GameOver)
                    .with_system(on_enter.system()))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver)
                    .with_system(keyboard_input.system()))
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver)
                    .with_system(cleanup.system()))
            ;
    }
}

fn create_resources(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    texture_assets: Res<loading::TextureAssets>,
) {
    commands.insert_resource(Sprites {
        press_space: materials.add(texture_assets.game_over_press_space.clone().into()),
        text: materials.add(texture_assets.game_over_text.clone().into()),
    });
}

fn on_enter(
    mut commands: Commands,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    sprites: Res<Sprites>,
) {
    commands
        .spawn()
        .insert(GameOver)
        .insert_bundle(SpriteBundle {
            material: sprites.text.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                scale: Vec3::ONE * 1.5,
                .. Default::default()
            },
            .. Default::default()
        });

    commands
        .spawn()
        .insert(GameOver)
        .insert_bundle(SpriteBundle {
            material: sprites.press_space.clone(),
            transform: Transform {
                translation: Vec3::new(0., -150., 10.),
                scale: Vec3::ONE * 0.7,
                .. Default::default()
            },
            .. Default::default()
        });

    audio.play(sounds.game_over.clone());
}

fn keyboard_input(
    mut kb: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if kb.just_pressed(KeyCode::Space) {
        log::debug!("kb.just_pressed(Space)");
        // .reset() space so Playing state doesn't think it's already pressed
        kb.reset(KeyCode::Space);
        state.set(GameState::Playing).unwrap();
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<GameOver>>,
) {
    for ent in query.iter() {
        commands
            .entity(ent)
            .despawn();
    }
}
