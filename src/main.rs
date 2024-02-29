enum Ressource {
    Energie,
    Mineral,
    LieuInteretScientifique,
}

struct Carte {
    largeur: usize,
    hauteur: usize,
}

enum TypeRobot {
    Explorateur,
    Collecteur,
    Visiteur,
}

struct Module {
    nomModule: String,
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
    nom: String,
    x: usize,
    y: usize
}

fn main() {
    let robot1 = Robot {
        id: 1,
        nom: String::from("Robot1"),
        pv_max: 100,
        type_robot: TypeRobot::Explorateur,
        vitesse: 0
    };

    let carte: Carte = Carte {
        largeur: 10,
        hauteur: 10
    };

    println!("Nom : {}", robot1.nom);
    for i in 0..carte.hauteur {
        for j in 0..carte.largeur {
            if i == 0 || i == carte.hauteur - 1 {
                print!("_");
            } else if j == 0 || j == carte.largeur - 1 {
                print!("|");
            } else {
                print!(" ");
            }
        }
        println!(); 
    }
}
