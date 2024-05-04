use bevy_kira_audio::prelude::*;
use bevy::prelude::*;

//Fonction pour démarrer la musique de fond
pub fn start_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("music.ogg")).looped();
}

pub fn encaissement_audio(asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
    audio.play(asset_server.load("encaissement.ogg"));
}

pub fn energy_audio(asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
    audio.play(asset_server.load("energy.ogg"));
}

