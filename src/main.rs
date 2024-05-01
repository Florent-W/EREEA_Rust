extern crate noise;

mod plugins;
mod components;
mod resources;
mod systems;

use bevy::window::WindowMode;
use bevy::{prelude::*};
use components::{assign_targets, collect_resources_system, discover_elements, move_robots_on_map_system, setup_bordures, setup_map, spawn_robots, update_robot_state, AffichageCasesNonDecouvertes, Compteur, CompteurRobotsSpawn, SeedResource, VitesseGlobale};
use systems::utilities::{request_nb_robots, request_seed_from_user};

use crate::systems::*;

fn main() {
    let seed_option = request_seed_from_user();
    let (width, height) = request_resolution_from_user();
    let nb_robots = request_nb_robots();

    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Essaim de Robots pour Exploration et Etude Astrobiologique".to_string(),
                    mode: WindowMode::Windowed,
                    resolution: (width, height).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(AffichageCasesNonDecouvertes(false))
        .insert_resource(VitesseGlobale { vitesse : 1 })
        .insert_resource(SeedResource { seed: seed_option })
        .insert_resource(Compteur { minerai: 0, energie: 0 })
        .insert_resource(CompteurRobotsSpawn { nombre: nb_robots })
        .add_systems(Startup, setup_map)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_legend)
        .add_systems(PostStartup, setup_bordures)
        .add_systems(PostStartup, spawn_robots)
        .add_systems(Update, move_robots_on_map_system)
        .add_systems(Update, toggle_cases_non_decouvertes)
        .add_systems(Update, toggle_vitesse)
        .add_systems(Update, toggle_fullscreen)
        .add_systems(Update, toggle_exit_game)
        .add_systems(Update, assign_targets)
        .add_systems(Update, adjust_visibility_system)
        .add_systems(Update, update_robot_state)
        .add_systems(Update, collect_resources_system)
        .add_systems(Update, move_camera_system)
        .add_systems(Update, zoom_camera_system)
        .add_systems(PostUpdate, discover_elements)
        .run();
}
