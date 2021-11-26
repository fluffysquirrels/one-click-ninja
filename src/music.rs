use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioSource};
use crate::{
    game_state::GameState,
    loading,
};

pub struct Plugin;

struct Sounds {
    playing_loop: Handle<AudioSource>,
}

struct MusicInstance {
    instance: bevy_kira_audio::InstanceHandle,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::CreateResources)
                    .with_system(create_resources.system()))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(start_music.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(on_update.system()))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                    .with_system(stop_music.system()))
            ;
    }
}

fn create_resources(
    mut commands: Commands,
    audio_assets: Res<loading::AudioAssets>,
) {
    commands.insert_resource(Sounds {
        playing_loop: audio_assets.playing_loop.clone(),
    });
}

fn start_music(
    mut commands: Commands,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    let instance = audio.play_looped(sounds.playing_loop.clone());
    commands.insert_resource(MusicInstance {
        instance,
    });
}

fn on_update(
    music_instance: Res<MusicInstance>,
) {
    log::trace!("Music pos: {}", music_instance.instance.position());
}

fn stop_music(
    audio: Res<Audio>,
) {
    audio.stop();
}
