extern crate noise;
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;
use std::{thread, time};

#[derive(PartialEq)]
enum Ressource {
    Energie,
    Mineral,
    LieuInteretScientifique,
}

struct Carte {
    largeur: usize,
    hauteur: usize,
    obstacles: Vec<Obstacle>,
    ressources: Vec<(Ressource, usize, usize)>,
    robots: Vec<Robot>,
    base: Vec<(i32, i32)>
}

enum TypeRobot {
    Explorateur,
    Collecteur,
    Visiteur,
}

struct Module {
    nom_module: String,
    partie: String
}

struct Robot {
    id: i32,
    nom: String,
    pv_max: i32,
    type_robot: TypeRobot,
    vitesse: i32,
    position_x: i32,
    position_y: i32
}

struct Obstacle {
    id: i32,
    x: usize,
    y: usize
}

fn generer_obstacles_ressources(carte: &mut Carte, seed: u32) {
    let perlin = Perlin::new(seed); 
    let mut rng = rand::thread_rng();

    carte.obstacles.clear();
    carte.ressources.clear();
    carte.base.clear();

    let base_x = rng.gen_range(0..carte.largeur);
    let base_y = rng.gen_range(0..carte.hauteur);
    carte.base.push((base_x as i32, base_y as i32));

    for i in 0..carte.hauteur {
        for j in 0..carte.largeur {
            if carte.base.iter().any(|&(bx, by)| bx as usize == j && by as usize == i) {
                continue;
            }

            let noise_value = perlin.get([(i as f64 * 0.1), (j as f64 * 0.1)]);
            let noise_normaliser = (noise_value + 1.0) / 2.0; 

            if noise_normaliser > 0.8 {
                carte.obstacles.push(Obstacle { id: rng.gen(), x: j, y: i });
            } else if noise_normaliser > 0.75 {
                carte.ressources.push((Ressource::Energie, j, i)); 
            } else if noise_normaliser > 0.72 {
                carte.ressources.push((Ressource::Mineral, j, i)); 
            } else if noise_normaliser > 0.7 {
                carte.ressources.push((Ressource::LieuInteretScientifique, j, i)); 
            }
        }
    }
}

fn main() {
    let mut carte: Carte = Carte {
        largeur: 20,
        hauteur: 20,
        obstacles: Vec::new(),
        ressources: Vec::new(),
        robots: Vec::new(),
        base: Vec::new()
    };

    let duree_tick = time::Duration::from_millis(20000);

    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen(); // Génération du seed

    println!("seed {}", seed);

    generer_obstacles_ressources(&mut carte, seed);

     if !carte.base.is_empty() {
        let (base_x, base_y) = carte.base[0]; 

        // Génération des robots
        let nombres_robots = 5;
        for id in 1..=nombres_robots {
            carte.robots.push(Robot {
                id,
                nom: format!("Robot{}", id),
                pv_max: 100,
                type_robot: if id % 2 == 0 { 
                    TypeRobot::Collecteur 
                } else { 
                    TypeRobot::Explorateur 
                },
                vitesse: 1,
                position_x: base_x, 
                position_y: base_y,
            });
        }
    } else {
        println!("Erreur : Aucune base n'a été générée.");
        return;
    }
    
    loop {        
        // Affichage de la carte avec les contours
        for i in 0..carte.hauteur {
            for j in 0..carte.largeur {
                if i == 0 || i == carte.hauteur - 1 { 
                    print!("-");
                } else if j == 0 || j == carte.largeur - 1 { 
                    print!("|");
                } else if carte.base.iter().any(|&(base_x, base_y)| base_x == j as i32 && base_y == i as i32) {
                    print!("B"); // Base
                } else if carte.obstacles.iter().any(|&Obstacle { x, y, .. }| x == j && y == i) {
                    print!("O"); // obstacle
                } else if carte.ressources.iter().any(|&(ref ressource, x, y)| x == j && y == i && *ressource == Ressource::Energie) {
                    print!("E"); // Energie
                } else if carte.ressources.iter().any(|&(ref ressource, x, y)| x == j && y == i && *ressource == Ressource::Mineral) {
                    print!("M"); // Mineral
                } else if carte.ressources.iter().any(|&(ref ressource, x, y)| x == j && y == i && *ressource == Ressource::LieuInteretScientifique) {
                    print!("L"); // Lieu d'Intérêt Scientifique
                } else {
                    print!(" "); // Espace vide
                }
            }
            println!();
        }
        thread::sleep(duree_tick); 
    }
}
