use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioSource};
use crate::{
    events::MusicTime,
    game_state::GameState,
    loading,
};

pub struct Plugin;

struct Sounds {
    playing_loop: TrackSettings,
}

struct MusicInstance {
    instance: bevy_kira_audio::InstanceHandle,
    track: TrackSettings,
}

#[derive(Clone)]
struct TrackSettings {
    audio: Handle<AudioSource>,
    start_offset: f64,
    bpm: f64,
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
    audio_assets: Res<loading::Sounds>,
) {
    commands.insert_resource(Sounds {
        playing_loop: TrackSettings {
            audio: audio_assets.playing_loop.clone(),
            start_offset: 0.,
            bpm: 160.,
        },
    });
}

fn start_music(
    mut commands: Commands,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    let track = &sounds.playing_loop;
    let instance = audio.play_looped(track.audio.clone());
    commands.insert_resource(MusicInstance {
        instance,
        track: track.clone(),
    });
}

fn on_update(
    mut music_time_writer: EventWriter<MusicTime>,
    audio: Res<Audio>,
    music_instance: Res<MusicInstance>,
) {
    let track = music_instance.track.clone();
    let pos = audio.state(music_instance.instance.clone()).position();
    if let Some(pos) = pos {
        let beat_secs = 60. / track.bpm;
        let bar_secs = beat_secs * 4.;
        let bar_offset = (pos - track.start_offset) % bar_secs;
        let beat_in_bar = (bar_offset / bar_secs) * 4.;
        let time = MusicTime {
            loop_position: pos,
            beat_in_bar: beat_in_bar,
        };
        log::trace!("MusicTime: {:?}", time);
        music_time_writer.send(time);
    }
}

fn stop_music(
    audio: Res<Audio>,
) {
    audio.stop();
}
