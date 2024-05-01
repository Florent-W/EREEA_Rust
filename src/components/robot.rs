use bevy::prelude::*;
use rand::Rng;
use crate::{update_text, CompteurRobotsSpawn, VitesseGlobale};

use super::{Base, Carte, Compteur, ElementCarte, ElementMap, EtatDecouverte, Position, Ressource, TexteEnergie, TexteMinerai, TexteVitesse};

const ROBOT_SPRITE: &str = "textures/robot.png";

#[derive(Component, PartialEq, Debug)]
pub enum TypeRobot {
    Explorateur,
    Collecteur,
    Visiteur,
}

#[derive(Component, PartialEq, Debug)]
pub enum RobotState {
    Exploring,
    Returning,
    AtBase,
}

#[derive(Component, Debug)]
pub struct Robot {
    id: u32,
    nom: String,
    pv_max: i32,
    type_robot: TypeRobot,
    vitesse: u32,
    timer: f32,
    target_position: Option<Position>,
    steps_moved: u32,
}

/***
 * Fonction d'ajout des robots sur la carte
 */
pub fn spawn_robots(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    base_query: Query<(&Base, &Position)>,
    vitesse_globale: Res<VitesseGlobale>,
    compteur_robots_spawn: Res<CompteurRobotsSpawn>
) {   
    let robot_texture_handle = asset_server.load(ROBOT_SPRITE);

    if let Some((_, base_position)) = base_query.iter().next() {
        for id in 1..=compteur_robots_spawn.nombre {
            let (type_robot, color, vitesse) = match id % 3 {
                0 => (TypeRobot::Explorateur, Some(Color::rgb(0.0, 1.0, 0.0)), 2),
                1 => (TypeRobot::Collecteur, Some(Color::rgb(0.0, 0.0, 1.0)), 1),
                _ => (TypeRobot::Visiteur, None, 1),
            };

            let robot_name = match type_robot {
                TypeRobot::Explorateur => format!("Explorateur{}", id),
                TypeRobot::Collecteur => format!("Collecteur{}", id),
                TypeRobot::Visiteur => format!("Visiteur{}", id),
            };

            // Cible aléatoire sur la map pour les robots
            let target_x: i32 = rand::thread_rng().gen_range(0..50) as i32;
            let target_y: i32 = rand::thread_rng().gen_range(0..50) as i32;

            let timer = 5.0 / (vitesse * vitesse_globale.vitesse) as f32;

            commands
                .spawn(SpriteBundle {
                    texture: robot_texture_handle.clone(),
                    sprite: Sprite {
                        color: color.unwrap_or(Color::WHITE),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        base_position.x as f32,
                        base_position.y as f32,
                        1.0,
                    ))
                    .with_scale(Vec3::splat(0.003)),
                    ..Default::default()
                })
                .insert(Robot {
                    id: id,
                    nom: robot_name,
                    pv_max: 100,
                    type_robot: type_robot,
                    vitesse: vitesse,
                    timer: timer,
                    target_position: Some(Position {
                        x: target_x,
                        y: target_y,
                    }),
                    steps_moved: 0,
                })
                .insert(Position {
                    x: base_position.x,
                    y: base_position.y,
                })
                .insert(RobotState::AtBase);
        }
    }
}

/***
 * Fonction pour déplacer les robots
 */
pub fn move_robots_on_map_system(
    mut query: Query<(
        Entity,
        &mut Position,
        &mut Transform,
        &mut Robot,
        &mut RobotState,
    )>,
    carte_query: Query<&Carte>,
    base_query: Query<(&Base, &Position), Without<Robot>>, // Exclure les Robots ici
    element_carte_query: Query<(&ElementCarte, &Position), Without<Robot>>, // Exclure les Robots ici
    time: Res<Time>,
    vitesse_globale: Res<VitesseGlobale>
) {
    let delta = time.delta_seconds();
    let carte = carte_query.single();
    let (_, base_pos) = base_query.single(); // Position de la base

    for (_entity, mut position, mut transform, mut robot, mut state) in query.iter_mut() {
        robot.timer -= delta;

        match *state {
            RobotState::Exploring => {
                if robot.timer <= 0.0 {
                    if let Some(target_position) = &robot.target_position {
                        if *position != *target_position {
                            // Vérifie si la position suivante est un obstacle
                            let next_x = (position.x + (target_position.x - position.x).signum())
                                % carte.largeur as i32;
                            let next_y = (position.y + (target_position.y - position.y).signum())
                                % carte.hauteur as i32;
                            let next_position = Position {
                                x: next_x,
                                y: next_y,
                            };

                            if element_carte_query.iter().any(|(elem, pos)| {
                                *pos == next_position
                                    && matches!(
                                        elem.element,
                                        ElementMap::Ressource(Ressource::Obstacle)
                                    )
                            }) {
                                // Trouve une nouvelle direction aléatoire pour éviter l'obstacle
                                robot.target_position = Some(Position {
                                    x: rand::thread_rng().gen_range(0..carte.largeur) as i32,
                                    y: rand::thread_rng().gen_range(0..carte.hauteur) as i32,
                                });
                            } else {
                                // Déplace normalement si aucune obstacle n'est détecté
                                position.x = next_x;
                                position.y = next_y;
                                robot.steps_moved += 1;
                            }
                        } else {
                            // Cible atteinte ou limite de déplacement atteinte
                            robot.target_position = None;
                            robot.steps_moved = 0;
                            *state = RobotState::Returning;
                        }
                    }
                    robot.timer = 1.0 / (robot.vitesse * vitesse_globale.vitesse) as f32;
                }
            }
            RobotState::Returning => {
                // Logique pour le retour à la base
                if robot.timer <= 0.0 {
                    if *position != *base_pos {
                        position.x = (position.x + (base_pos.x - position.x).signum())
                            % carte.largeur as i32;
                        position.y = (position.y + (base_pos.y - position.y).signum())
                            % carte.hauteur as i32;
                    } else {
                        *state = RobotState::AtBase;
                        robot.timer = 5.0; // Attente à la base
                    }
                    robot.timer = 1.0 / (robot.vitesse * vitesse_globale.vitesse) as f32;
                }
            }
            RobotState::AtBase => {
                // Logique pour envoyer le robot explorer à nouveau
                if robot.timer <= 0.0 {
                    let target_x = rand::thread_rng().gen_range(0..carte.largeur) as i32;
                    let target_y = rand::thread_rng().gen_range(0..carte.hauteur) as i32;
                    robot.target_position = Some(Position {
                        x: target_x,
                        y: target_y,
                    });
                    robot.steps_moved = 0;

                    *state = RobotState::Exploring;
                    robot.timer = 1.0 / (robot.vitesse * vitesse_globale.vitesse) as f32;
                }
            }
        }

        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;
    }
}

/***
 * Fonction pour changer les états des robots
 */
pub fn update_robot_state(
    mut query: Query<(&mut Robot, &Position, &mut RobotState)>,
    base_query: Query<(&Base, &Position)>,
) {
    let (_, base_pos) = base_query.single();

    for (robot, robot_pos, mut state) in query.iter_mut() {
        if robot.target_position.is_none() && *state == RobotState::Exploring {
            *state = RobotState::Returning;
        }
        if *state == RobotState::Returning && robot_pos.x == base_pos.x && robot_pos.y == base_pos.y
        {
            *state = RobotState::AtBase;
        }
    }
}

/***
 * Fonction de collecte des ressources si un robot est sur la même position qu'une ressource
 */
pub fn collect_resources_system(
    mut commands: Commands,
    mut robot_query: Query<(Entity, &mut Robot, &Position)>,
    mut element_carte_query: Query<(Entity, &mut ElementCarte, &Position)>,
    mut query_energie: Query<&mut Text, (With<TexteEnergie>, Without<TexteMinerai>, Without<TexteVitesse>)>,
    mut query_minerai: Query<&mut Text, (With<TexteMinerai>, Without<TexteEnergie>, Without<TexteVitesse>)>,
    mut compteur: ResMut<Compteur>,
) {
    for (_robot_entity, robot, robot_position) in robot_query.iter_mut() {
        // println!("{:?}", robot.type_robot);
        if robot.type_robot == TypeRobot::Collecteur {
       // println!("Checking robot {} at position {:?}", robot.nom, robot_position); 
        let mut resource_collected = false; 
        for (entity, mut element_carte, resource_position) in element_carte_query.iter_mut() {
            if robot_position == resource_position {
                resource_collected = true; // La ressource a été trouvée à la même position que le robot
                match element_carte.element {
                    ElementMap::Ressource(Ressource::Energie) => {
                      //  println!("Robot {} collected energy at position {:?}", robot.nom, robot_position);
                        compteur.energie += 1;
                        commands.entity(entity).despawn();
                        update_text(&compteur, &mut query_energie, &mut query_minerai);
                    }
                    ElementMap::Ressource(Ressource::Mineral) => {
                      //  println!("Robot {} collected mineral at position {:?}", robot.nom, robot_position); 
                        compteur.minerai += 1;
                        commands.entity(entity).despawn();
                        update_text(&compteur, &mut query_energie, &mut query_minerai);
                    }
                    ElementMap::Ressource(Ressource::LieuInteretScientifique) => {
                      //  println!("Robot {} discovered a place of interest at position {:?}", robot.nom, robot_position); 
                    }
                    _ => {
                    }
                }
                if element_carte.est_decouvert == EtatDecouverte::NonDecouvert {
                    element_carte.est_decouvert = EtatDecouverte::EnAttente;
                }
            }
        }
        if !resource_collected {
            // println!("Robot {} did not collect any resources at position {:?}", robot.nom, robot_position);
        }
    }
    }
}

/***
 * Fonction pour ajouter des positions de cible aux robots
 */
pub fn assign_targets(
    mut query: Query<(&mut Robot, &Position)>,
    map_query: Query<&Carte>,
    element_carte_query: Query<(&ElementCarte, &Position)>, 
) {
    if let Ok(_carte) = map_query.get_single() {
        for (mut robot, _robot_pos) in query.iter_mut() {
            if robot.type_robot == TypeRobot::Explorateur && robot.target_position.is_none() {
                let mut target_position: Option<Position> = None;
                let mut available_positions = Vec::new();

                // Collecter toutes les positions non découvertes
                for (element_carte, pos) in element_carte_query.iter() {
                    if element_carte.est_decouvert == EtatDecouverte::NonDecouvert {
                        available_positions.push(*pos);
                    }
                }

                // Sélectionner aléatoirement une de ces positions disponibles
                if !available_positions.is_empty() {
                    let mut rng = rand::thread_rng();
                    target_position =
                        Some(available_positions[rng.gen_range(0..available_positions.len())]);
                }

                // Si une position valide est trouvée, l'assigner au robot
                if let Some(pos) = target_position {
                    robot.target_position = Some(pos);
                    robot.steps_moved = 0;
                }
            }
        }
    }
}

/***
 * Fonction qui découvre les cases lorsqu'un robot est à la base
 */
pub fn discover_elements(
    robot_query: Query<&RobotState, With<Robot>>,
    mut elements_query: Query<(&mut ElementCarte, &Position)>,
) {
    // On met à jour quand le robot est à la base
    let robot_at_base = robot_query
        .iter()
        .any(|state| matches!(state, RobotState::AtBase));

    if robot_at_base {
        for (mut element_carte, _) in elements_query.iter_mut() {
            if element_carte.est_decouvert == EtatDecouverte::EnAttente {
                element_carte.est_decouvert = EtatDecouverte::Decouvert;
            }
        }
    }
}
