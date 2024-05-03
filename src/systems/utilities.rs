use bevy::{app::AppExit, prelude::*};
use std::io;

use crate::components::{AffichageCasesNonDecouvertes, ElementCarte, EtatDecouverte};

/***
 * Fonction pour demander le nombre de robots à faire spawn
 */
pub fn request_nb_robots() -> u32 {
    println!("Choisissez le nombre de robot :");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur dans la lecture de la ligne");

    println!("{input}");

    let trimmed = input.trim();

    let nb_robots = trimmed.parse::<u32>().unwrap_or_else(|_| {
        println!("Mauvaise valeur dans le choix du nombre de robot. La simulation commencera à 5 robots.");
        5 
    });

    if nb_robots > 30 {
        println!("Le nombre de robots ne peut pas dépasser 30. La simulation commencera à 30 robots.");
        30
    } else {
        nb_robots
    }
}

/***
 * Permet de demander un seed à l'utilisateur
 * */
 pub fn request_seed_from_user() -> Option<u32> {
    println!(
        "Veuillez entrer un seed (nombre) ou appuyez sur entrer pour prendre un seed aléatoire:"
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur dans la lecture de la ligne");

    if input.trim().is_empty() {
        None
    } else {
        input.trim().parse::<u32>().ok()
    }
}

/***
 * Permet de demander la taille de la map à l'utilisateur
 */
 pub fn request_size_map_from_user() -> Option<u32> {
    println!(
        "Veuillez entrer la taille de la map :"
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur dans la lecture de la ligne");

    if input.trim().is_empty() {
        None
    } else {
        input.trim().parse::<u32>().ok()
    }
}

/***
  * Fonction pour voir toutes les cases
 */
pub fn toggle_cases_non_decouvertes(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut affichage: ResMut<AffichageCasesNonDecouvertes>,
) {
    // Bascule l'état d'affichage quand la touche Tab est pressée
    if keyboard_input.just_pressed(KeyCode::Tab) {
        affichage.0 = !affichage.0;
    }
}

/***
 * Fonction qui change l'état des cases
 */
pub fn adjust_visibility_system(
    affichage_cases: Res<AffichageCasesNonDecouvertes>,
    mut query: Query<(&ElementCarte, &mut Visibility)>,
) {
    for (element_carte, mut visibility) in query.iter_mut() {
        if !affichage_cases.0
            && (element_carte.est_decouvert == EtatDecouverte::NonDecouvert
                || element_carte.est_decouvert == EtatDecouverte::EnAttente)
        {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}

/***
 * Fonction pour quitter le jeu
 */
pub fn toggle_exit_game(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    } 
}


/***
 * Fonction pour demander la résolution à l'utilisateur
 */
pub fn request_resolution_from_user() -> (f32, f32) {
    println!("Choisissez la résolution :");
    println!("1. 1280x720");
    println!("2. 1920x1080");
    println!("3. Autre (entrez la résolution personnalisée sous la forme largeur hauteur)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur dans la lecture de la ligne");

    match input.trim() {
        "1" => (1280.0, 720.0),
        "2" => (1920.0, 1080.0),
        "3" => {
            println!("Entrez la résolution personnalisée (largeur hauteur) :");
            let mut custom_input = String::new();
            io::stdin()
                .read_line(&mut custom_input)
                .expect("Erreur dans la lecture de la ligne");

            let parts: Vec<&str> = custom_input.trim().split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(width), Ok(height)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>())
                {
                    (width, height)
                } else {
                    println!(
                        "Entrée invalide, utilisation de la résolution par défaut (1280x720)."
                    );
                    (1280.0, 720.0)
                }
            } else {
                println!("Entrée invalide, utilisation de la résolution par défaut (1280x720).");
                (1280.0, 720.0)
            }
        }
        _ => {
            println!("Option invalide, utilisation de la résolution par défaut (1280x720).");
            (1280.0, 720.0)
        }
    }
}

