extern crate noise;

mod components;
mod systems;

use bevy::window::WindowMode;
use bevy::prelude::*;
use components::{assign_targets, collect_resources_system, discover_elements, move_robots_on_map_system, setup_bordures, setup_map, spawn_robots, update_robot_state, AffichageCasesNonDecouvertes, BorduresActive, Compteur, SeedResource, SizeMap, VitesseGlobale};
use systems::utilities::{request_nb_robots, request_seed_from_user};

use crate::systems::*;

fn main() {
    let seed_option = request_seed_from_user();
    let (width, height) = request_resolution_from_user();
    let nb_robots = request_nb_robots();
    let size = request_size_map_from_user();

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
        .insert_resource(SizeMap { length: size, height: size })
        .insert_resource(BorduresActive(true))
        // On fait spawn le nombre de ressources qu'il faut pour faire apparaitre le nombre de robots
        .insert_resource(Compteur { minerai: 5 * nb_robots, energie: 3 * nb_robots, total_robots: 0 })
        .add_systems(Startup, setup_map)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_legend)
        .add_systems(PostStartup, setup_bordures)
        .add_systems(Update, spawn_robots)
        .add_systems(Update, move_robots_on_map_system)
        .add_systems(Update, toggle_cases_non_decouvertes)
        .add_systems(Update, toggle_bordures)
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
