enum Ressource {
    Energie,
    Mineral,
    LieuInteretScientifique,
}

enum TypeRobot {
    Explorateur,
    Collecteur,
    Visiteur,
}

struct Robot {
    id: i32,
    nom: String,
    pv_max: i32,
    type_robot: TypeRobot
}

struct Obstacle {
    id: i32,
    nom: String,
}

fn main() {
    let robot1 = Robot {
        id: 1,
        nom: String::from("Robot1"),
        pv_max: 100,
        type_robot: TypeRobot::Explorateur
    };

    println!("Nom : {}", robot1.nom);
}
