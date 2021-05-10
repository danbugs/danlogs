use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, OggFormat, Mp3Format, Source, SourceHandle},
    ecs::prelude::{World, WorldExt},
};

use std::iter::Cycle;
use std::vec::IntoIter;

pub const AUDIO_MUSIC : &[&str] = &["./audio/bgm.mp3"];
pub const AUDIO_BOUNCE : &str = "./audio/hit.ogg";
pub const AUDIO_SCORE : &str = "./audio/point.ogg";

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>
}

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle
}

pub enum Format {
    Ogg,
    Mp3
}

fn load_audio_track(loader: &Loader, world: &World, file: &str, kind: Format) -> SourceHandle {
    let load: SourceHandle = match kind {
        Ogg => loader.load(file, OggFormat, (), &world.read_resource()),
        Mp3 => loader.load(file, Mp3Format, (), &world.read_resource()),
    };

    load
}

pub fn initialize_audio(world: &mut World) {
    let (sound_effects, music) = {
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25);
        let loader = world.read_resource::<Loader>();
        let music = AUDIO_MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, file, Format::Mp3))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        let music = Music { music };

        let sounds = Sounds {
            score_sfx: load_audio_track(&loader, &world, AUDIO_SCORE, Format::Ogg),
            bounce_sfx: load_audio_track(&loader, &world, AUDIO_BOUNCE, Format::Ogg),
        };

        (sounds, music)
    };

    world.insert(sound_effects);
    world.insert(music);
}

pub fn play_bounce(
    sounds: &Sounds,
    storage: &AssetStorage<Source>,
    output: Option<&Output>
) {
    if let Some(output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_score(
    sounds: &Sounds,
    storage: &AssetStorage<Source>,
    output: Option<&Output>
) {
    if let Some(output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}
