use bevy::prelude::*;
use crate::{
    game_state::GameState,
    loading,
};

struct Menu;

struct Sprites {
    text: Handle<ColorMaterial>,
    click_here: Handle<ColorMaterial>,
}

/// Should have a menu but currently just goes directly to playing
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Setup)
                    .with_system(create_resources.system()))
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(on_enter.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(keyboard_input.system()))
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
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
        text: materials.add(texture_assets.menu_text.clone().into()),
        click_here: materials.add(texture_assets.menu_click_here.clone().into()),
    });
}

fn on_enter(
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
    commands
        .spawn()
        .insert(Menu)
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
        .insert(Menu)
        .insert_bundle(SpriteBundle {
            material: sprites.click_here.clone(),
            transform: Transform {
                translation: Vec3::new(0., -200., 10.),
                scale: Vec3::ONE * 0.7,
                .. Default::default()
            },
            .. Default::default()
        });
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
    query: Query<Entity, With<Menu>>,
) {
    for ent in query.iter() {
        commands
            .entity(ent)
            .despawn();
    }
}
