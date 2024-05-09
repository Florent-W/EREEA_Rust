use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_kira_audio::{Audio, AudioControl};

// Fonction pour démarrer la musique de fond
pub fn start_audio(asset_server: Res<AssetServer>, audio: ResMut<Audio>) {
    let music_handle: Handle<AudioSource> = asset_server.load("musics/evolution.ogg");

    audio.set_volume(0.09);
    audio.play(music_handle).looped();
}

pub fn encaissement_audio(asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
    let effect_handle: Handle<AudioSource> = asset_server.load("musics/encaissement.ogg");
    audio.play(effect_handle).with_volume(0.4);
}
