use bevy::prelude::*;

use crate::components::{AffichageCasesNonDecouvertes, Compteur, CompteurRobotsSpawn, SeedResource, VitesseGlobale};

// Fonction pour initialiser les ressources
pub fn setup_resources(
    commands: &mut Commands,
    initial_speed: u32,
    seed: Option<u32>,
    initial_robot_count: u32,
) {
    commands.insert_resource(Compteur {
        minerai: 0,
        energie: 0,
    });
    commands.insert_resource(VitesseGlobale {
        vitesse: initial_speed,
    });
    commands.insert_resource(CompteurRobotsSpawn {
        nombre: initial_robot_count,
    });
    commands.insert_resource(AffichageCasesNonDecouvertes(false));
    commands.insert_resource(SeedResource {
        seed,
    });
}