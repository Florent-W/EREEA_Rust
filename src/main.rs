extern crate noise;
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

enum Ressource {
    Energie,
    Mineral,
    LieuInteretScientifique,
}

struct Carte {
    largeur: usize,
    hauteur: usize,
    obstacles: Vec<Obstacle>,
    ressources: Vec<(Ressource, usize, usize)>
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
    vitesse: i32
}

struct Obstacle {
    id: i32,
    x: usize,
    y: usize
}

fn generer_obstacles_ressources(carte: &mut Carte, seed: u32) {
    let perlin = Perlin::new(seed); 
    let mut obstacle_id = 0;
    for i in 0..carte.hauteur {
        for j in 0..carte.largeur {
            let noise_value = perlin.get([(i as f64 * 0.1), (j as f64 * 0.1)]);
            let noise_normaliser = (noise_value + 1.0) / 2.0; // Normalise la valeur entre 0 et 1
            print!("{}", noise_normaliser);

            if noise_normaliser > 0.8 { // Noise pour les obstacles
                carte.obstacles.push(Obstacle { id: obstacle_id, x: j, y: i });
                obstacle_id += 1;
            } else if noise_normaliser > 0.7 { // Noise pour les ressources
                carte.ressources.push((Ressource::Energie, j, i)); 
            }
        }
    }
}

fn main() {
    let robot1 = Robot {
        id: 1,
        nom: String::from("Robot1"),
        pv_max: 100,
        type_robot: TypeRobot::Explorateur,
        vitesse: 0
    };

    let mut carte: Carte = Carte {
        largeur: 100,
        hauteur: 100,
        obstacles: Vec::new(),
        ressources: Vec::new()
    };

    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen(); // Génération du seed

    generer_obstacles_ressources(&mut carte, seed);

    print!("seed {}", seed);

    // Affichage de la carte avec les contours
    for i in 0..carte.hauteur {
        for j in 0..carte.largeur {
            if i == 0 || i == carte.hauteur - 1 { 
                print!("-");
            } else if j == 0 || j == carte.largeur - 1 { 
                print!("|");
            } else if carte.obstacles.iter().any(|obstacle| obstacle.x == j && obstacle.y == i) {
                print!("O"); // obstacle
            } else if carte.ressources.iter().any(|&(_, x, y)| x == j && y == i) {
                print!("R"); // ressource
            } else {
                print!(" "); // Espace vide
            }
        }
        println!();
    }
}
