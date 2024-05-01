use bevy::prelude::*;
use bevy::window::WindowMode;

/***
 * Configure les plugins à utiliser
 */
pub fn get_default_plugins(width: f32, height: f32) -> impl PluginGroup {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Essaim de Robots pour Exploration et Etude Astrobiologique".to_string(),
            mode: WindowMode::Windowed,
            resolution: (width, height).into(),
            ..default()
        }),
        ..default()
    })
}