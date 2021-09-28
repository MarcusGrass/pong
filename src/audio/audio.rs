use amethyst::{
    assets::Loader,
    audio::{OggFormat, SourceHandle},
    ecs::{World, WorldExt},
};
use amethyst::assets::{AssetStorage};
use amethyst::audio::{Source, AudioSink};
use amethyst::audio::output::Output;
use std::{iter::Cycle, vec::IntoIter};
use crate::persistence::{AudioSettings, Settings};

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";
const BOO_SOUND: &str = "audio/boo.ogg";

const MUSIC_TRACKS: &[&str] = &[
    "audio/track1.ogg",
    "audio/track2.ogg",
    "audio/track3.ogg",
];

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
    pub boo_sfx: SourceHandle,
}

pub fn play_bounce_sound(settings: &AudioSettings, sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, settings.effects_volume)
        }
    }
}

pub fn play_score_sound(settings: &AudioSettings, sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, settings.effects_volume);
        }
        if let Some(sound) = storage.get(&sounds.boo_sfx) {
            output.play_once(sound, settings.effects_volume);
        }
    }
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}
pub fn initialise_audio(world: &mut World, settings: &Settings) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(settings.audio_settings.music_volume);

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        let sound = Sounds {
            bounce_sfx: load_audio_track(&loader, &world, BOUNCE_SOUND),
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND),
            boo_sfx: load_audio_track(&loader, &world, BOO_SOUND),
        };

        (sound, music)
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
    world.insert(music);
}
