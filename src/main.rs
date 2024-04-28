extern crate noise;

use bevy::app::AppExit;
use bevy::ecs::query;
use bevy::window::WindowMode;
use bevy::{input::mouse::MouseWheel, prelude::*};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::io;

const ENERGIE_SPRITE: &str = "textures/energie.png";
const MINERAL_SPRITE: &str = "textures/minerai.png";
const LIEU_INTERET_SPRITE: &str = "textures/lieu.png";
const BASE_SPRITE: &str = "textures/base.png";
const ROBOT_SPRITE: &str = "textures/robot.png";
const OBSTACLE_SPRITE: &str = "textures/obstacle.png";

#[derive(Component, PartialEq, Debug)]
enum ElementMap {
    Ressource(Ressource),
    ElementGeographique(ElementGeographique),
}

#[derive(Component, Debug, PartialEq)]
enum Ressource {
    Energie,
    Mineral,
    LieuInteretScientifique,
    Obstacle,
}

#[derive(Component, Debug, PartialEq)]
enum ElementGeographique {
    Herbe,
    Terre,
    Eau,
    Sable,
}

#[derive(Component, Debug, PartialEq)]
enum EtatDecouverte {
    NonDecouvert,
    EnAttente,
    Decouvert,
}

#[derive(Component, Debug)]
struct Carte {
    largeur: usize,
    hauteur: usize,
}

#[derive(Component, PartialEq, Debug)]
enum TypeRobot {
    Explorateur,
    Collecteur,
    Visiteur,
}

enum ResolutionOption {
    Resolution1280x720,
    Resolution1920x1080,
    CustomResolution(f32, f32),
}

#[derive(Resource, Debug)]
struct AffichageCasesNonDecouvertes(bool);

#[derive(Resource, Debug)]
struct SeedResource {
    seed: Option<u32>,
}

#[derive(Component, PartialEq, Debug)]
enum RobotState {
    Exploring,
    Returning,
    AtBase,
}

#[derive(Clone)]
enum TextureOrColor {
    Texture(Handle<Image>),
    Color(Color),
}

#[derive(Component, Debug)]
struct Robot {
    id: u32,
    nom: String,
    pv_max: i32,
    type_robot: TypeRobot,
    vitesse: u32,
    timer: f32,
    target_position: Option<Position>,
    steps_moved: u32,
}

#[derive(Resource, Debug)]
struct Compteur {
    minerai: u32,
    energie: u32
}

#[derive(Resource, Debug)]
struct VitesseGlobale {
    vitesse: u32
}

#[derive(Resource, Debug)]
struct CompteurRobotsSpawn {
    nombre: u32
}

#[derive(Component)]
struct TexteEnergie;

#[derive(Component)]
struct TexteMinerai;

#[derive(Component)]
struct TexteVitesse;

#[derive(Component, Debug)]
struct ElementCarte {
    element: ElementMap,
    est_decouvert: EtatDecouverte,
}

#[derive(Component, PartialEq, Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
struct Bordure;

#[derive(Component, Debug)]
struct Base;

/***
 * Fonction pour la caméra
 */
fn setup_camera(mut commands: Commands, center_x: f32, center_y: f32) {
    let zoom_level = 0.05;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(center_x, center_y, 10.0)
            .with_scale(Vec3::new(zoom_level, zoom_level, 1.0)),
        ..default()
    });
    commands.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)));
}

/***
 * Fonction pour charger la map
 */
fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>, seed_res: Res<SeedResource>) {
    // Charger les textures pour les différents éléments de la carte
    let energie_texture_handle = asset_server.load(ENERGIE_SPRITE);
    let mineral_texture_handle = asset_server.load(MINERAL_SPRITE);
    let lieu_interet_texture_handle = asset_server.load(LIEU_INTERET_SPRITE);
    let base_handle = asset_server.load(BASE_SPRITE);
    let obstacle_handle = asset_server.load(OBSTACLE_SPRITE);

    // Dimensions de la carte
    let largeur = 50;
    let hauteur = 50;

    // Créer l'entité de la carte avec sa position de base
    commands.spawn((Carte { largeur, hauteur }, Position { x: 0, y: 0 }));

    let seed = seed_res.seed.unwrap_or_else(|| {
        let random_seed = rand::thread_rng().gen();
        println!("Pas de seed donné");
        random_seed
    });
    println!("Seed utilisé : {}", seed);
    let perlin = Perlin::new(seed);

    // Génération des éléments de la carte en fonction de la valeur du noise
    for y in 0..hauteur {
        for x in 0..largeur {
            let position = Position {
                x: x as i32,
                y: y as i32,
            };
            let noise_value = perlin.get([x as f64 * 0.08, y as f64 * 0.08]);
            let noise_normalised = (noise_value + 1.0) / 2.0;

            // Déterminer quel type de ressource générer en fonction de la valeur du bruit
            let sprite = match noise_normalised {
                n if n > 0.8 => Some((
                    ElementMap::Ressource(Ressource::Obstacle),
                    TextureOrColor::Texture(obstacle_handle.clone()),
                    0.0015,
                )),
                n if n > 0.75 => Some((
                    ElementMap::Ressource(Ressource::Energie),
                    TextureOrColor::Texture(energie_texture_handle.clone()),
                    0.0015,
                )),
                n if n > 0.72 => Some((
                    ElementMap::Ressource(Ressource::Mineral),
                    TextureOrColor::Texture(mineral_texture_handle.clone()),
                    0.0012,
                )),
                n if n > 0.7 => Some((
                    ElementMap::Ressource(Ressource::LieuInteretScientifique),
                    TextureOrColor::Texture(lieu_interet_texture_handle.clone()),
                    0.0015,
                )),
                n if n >= 0.6 => Some((
                    ElementMap::ElementGeographique(ElementGeographique::Herbe),
                    TextureOrColor::Color(Color::rgb(0.5, 0.75, 0.3)),
                    1.0,
                )),
                n if n > 0.4 && n < 0.6 => Some((
                    ElementMap::ElementGeographique(ElementGeographique::Terre),
                    TextureOrColor::Color(Color::rgb(0.69, 0.62, 0.541)),
                    1.0,
                )),
                n if n > 0.2 && n <= 0.4 => Some((
                    ElementMap::ElementGeographique(ElementGeographique::Sable),
                    TextureOrColor::Color(Color::rgb(0.76, 0.69, 0.5)),
                    1.0,
                )),
                n if n >= 0.0 => Some((
                    ElementMap::ElementGeographique(ElementGeographique::Eau),
                    TextureOrColor::Color(Color::rgb(0.4, 0.5, 0.8)),
                    1.0,
                )),
                _ => None,
            };

            if let Some((element, texture_or_color, taille)) = sprite {
                let sprite_bundle = match texture_or_color {
                    TextureOrColor::Texture(texture_handle) => SpriteBundle {
                        texture: texture_handle,
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0))
                            .with_scale(Vec3::splat(taille)),
                        visibility: Visibility::Visible,
                        ..Default::default()
                    },
                    TextureOrColor::Color(color) => SpriteBundle {
                        sprite: Sprite {
                            color,
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0))
                            .with_scale(Vec3::splat(taille)),
                        visibility: Visibility::Visible,
                        ..Default::default()
                    },
                };

                commands
                    .spawn(sprite_bundle)
                    .insert(ElementCarte {
                        element,
                        est_decouvert: EtatDecouverte::NonDecouvert,
                    })
                    .insert(position);
            }
        }
    }

    // Ajout de la base sur la carte
    let mut base_x: i32;
    let mut base_y: i32;
    loop {
        base_x = rand::thread_rng().gen_range(0..largeur) as i32;
        base_y = rand::thread_rng().gen_range(0..hauteur) as i32;

        // Calcule la valeur du bruit pour la position générée
        let noise_value = perlin.get([base_x as f64 * 0.1, base_y as f64 * 0.1]);
        let noise_normalised = (noise_value + 1.0) / 2.0;

        // Vérifie si la position n'est pas un obstacle
        if noise_normalised <= 0.8 {
            break;
        }
    }

    // Place la base à la position valide trouvée
    commands
        .spawn(SpriteBundle {
            texture: base_handle,
            transform: Transform::from_translation(Vec3::new(base_x as f32, base_y as f32, 0.0))
                .with_scale(Vec3::splat(0.002)),
            ..Default::default()
        })
        .insert(Base)
        .insert(Position {
            x: base_x,
            y: base_y,
        });

    // Centre la caméra sur la base
    setup_camera(commands, base_x as f32, base_y as f32);
}

/***
 * Fonction pour ajouter les bordures
 */
fn setup_bordures(mut commands: Commands, query: Query<(&Carte, &Position)>) {
    for (carte, carte_position) in query.iter() {
        let bordure_couleur = Color::BLACK;
        let epaisseur_bordure = 0.05;
        let taille_case = 1.0;
        println!("{}", carte.hauteur);
        for y in 0..carte.hauteur {
            for x in 0..carte.largeur {
                let x_pos = x as f32 + carte_position.x as f32 * taille_case;
                let y_pos = y as f32 + carte_position.y as f32 * taille_case;

                // Créer les bordures verticales
                if x < carte.largeur - 1 {
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: bordure_couleur,
                            custom_size: Some(Vec2::new(epaisseur_bordure, taille_case)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(x_pos + 0.5 * taille_case, y_pos, 2.0),
                        ..Default::default()
                    });
                }

                // Créer les bordures horizontales
                if y < carte.hauteur - 1 {
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: bordure_couleur,
                            custom_size: Some(Vec2::new(taille_case, epaisseur_bordure)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(x_pos, y_pos + 0.5 * taille_case, 2.0), // Ajustez le Z pour s'assurer qu'il est visible
                        ..Default::default()
                    });
                }
            }
        }
    }
}

/***
 * Fonction pour demander le nombre de robots à faire spawn
 */
fn request_nb_robots() -> u32 {
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
 * Fonction d'ajout des robots sur la carte
 */
fn spawn_robots(
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
 * Fonction de collecte des ressources si un robot est sur la même position qu'une ressource
 */
fn collect_resources_system(
    mut commands: Commands,
    mut robot_query: Query<(Entity, &mut Robot, &Position)>,
    mut element_carte_query: Query<(Entity, &mut ElementCarte, &Position)>,
    mut query_energie: Query<&mut Text, (With<TexteEnergie>, Without<TexteMinerai>, Without<TexteVitesse>)>,
    mut query_minerai: Query<&mut Text, (With<TexteMinerai>, Without<TexteEnergie>, Without<TexteVitesse>)>,
    mut compteur: ResMut<Compteur>,
) {
    for (robot_entity, mut robot, robot_position) in robot_query.iter_mut() {
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
                if (element_carte.est_decouvert == EtatDecouverte::NonDecouvert) {
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

fn update_robot_state(
    mut commands: Commands,
    mut query: Query<(&mut Robot, &Position, &mut RobotState)>,
    base_query: Query<(&Base, &Position)>,
) {
    let (_, base_pos) = base_query.single();

    for (mut robot, robot_pos, mut state) in query.iter_mut() {
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
 * Fonction pour déplacer la caméra avec les touches directionnelles
 */
fn move_camera_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let camera_speed = 10.0;

    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= camera_speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += camera_speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += camera_speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= camera_speed * time.delta_seconds();
        }
    }
}

/***
 * Permet de demander un seed à l'utilisateur
 * */
fn request_seed_from_user() -> Option<u32> {
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

fn toggle_cases_non_decouvertes(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut affichage: ResMut<AffichageCasesNonDecouvertes>,
) {
    // Bascule l'état d'affichage quand la touche Tab est pressée
    if keyboard_input.just_pressed(KeyCode::Tab) {
        affichage.0 = !affichage.0;
    }
}

/*
 *** Fonction pour augmenter ou diminuer la vitesse globale de l'application et du parcours des robots
 */ 
fn toggle_vitesse(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut vitesse_globale: ResMut<VitesseGlobale>,
    mut query_vitesse: Query<&mut Text, (With<TexteVitesse>, Without<TexteEnergie>, Without<TexteMinerai>)>,
) {
    if keyboard_input.just_pressed(KeyCode::F2) && vitesse_globale.vitesse < 150 {
        vitesse_globale.vitesse += 1;
        update_text_vitesse(&mut query_vitesse, vitesse_globale)
    }
    else if keyboard_input.just_pressed(KeyCode::F1) && vitesse_globale.vitesse > 1 {
        vitesse_globale.vitesse -= 1;
        update_text_vitesse(&mut query_vitesse, vitesse_globale)
    }
}

fn adjust_visibility_system(
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
 * Fonction pour faire un zoom avec la caméra
 */
fn zoom_camera_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut zoom_change = 0.0;
    for event in mouse_wheel_events.read() {
        zoom_change += event.y * 0.01;
    }

    if zoom_change != 0.0 {
        for mut transform in query.iter_mut() {
            transform.scale *= Vec3::new(1.0 + zoom_change, 1.0 + zoom_change, 1.0);
            transform.scale = transform.scale.clamp(Vec3::splat(0.03), Vec3::splat(5.0));
        }
    }
}

/***
 * Fonction pour déplacer les robots
 */
fn move_robots_on_map_system(
    mut commands: Commands,
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

    for (entity, mut position, mut transform, mut robot, mut state) in query.iter_mut() {
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

fn discover_elements(
    mut commands: Commands,
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

fn assign_targets(
    mut commands: Commands,
    mut query: Query<(&mut Robot, &Position)>,
    map_query: Query<&Carte>,
    element_carte_query: Query<(&ElementCarte, &Position)>, // Query pour les informations des cases
) {
    if let Ok(carte) = map_query.get_single() {
        for (mut robot, robot_pos) in query.iter_mut() {
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
 * Fonction pour ajouter l'interface
 */
fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
)
{
    let font = asset_server.load("polices/RobotoMono.ttf");

    commands.spawn(TextBundle {
        text: Text::from_sections([
            TextSection::new(
                "Énergies: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::GREEN,
                }
            )
        ])
        .with_justify(JustifyText::Left),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    })
    .insert(TexteEnergie);

    commands.spawn(TextBundle {
        text: Text::from_sections([
            TextSection::new(
                "Minerais: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::PURPLE,
                }
            )
        ])
        .with_justify(JustifyText::Left),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0), 
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    })
    .insert(TexteMinerai);  

    commands.spawn(TextBundle {
        text: Text::from_sections([
            TextSection::new(
                "Vitesse: x",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            ),
            TextSection::new(
                "1",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            )
        ])
        .with_justify(JustifyText::Right),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0), 
            right: Val::Px(25.0),
            ..default()
        },
        ..default()
    })
    .insert(TexteVitesse);    
}

/***
 * Fonction pour mettre à jour le texte des compteurs
 */
fn update_text(
    compteur: &Compteur,
    mut query_energie: &mut Query<&mut Text, (With<TexteEnergie>, Without<TexteMinerai>, Without<TexteVitesse>)>,
    mut query_minerai: &mut Query<&mut Text, (With<TexteMinerai>, Without<TexteEnergie>, Without<TexteVitesse>)>,
) {
    if let Ok(mut texte_energie) = query_energie.get_single_mut() {
        texte_energie.sections[1].value = compteur.energie.to_string();
    }
    if let Ok(mut texte_minerai) = query_minerai.get_single_mut() {
        texte_minerai.sections[1].value = compteur.minerai.to_string();
    } 
}

/***
 * Fonction pour mettre à jour le texte de la vitesse
 */
fn update_text_vitesse(
    query_vitesse: &mut Query<&mut Text, (With<TexteVitesse>, Without<TexteEnergie>, Without<TexteMinerai>)>,
    vitesse_globale: ResMut<VitesseGlobale>
) {
    if let Ok(mut texte_vitesse) = query_vitesse.get_single_mut() {
        texte_vitesse.sections[1].value = vitesse_globale.vitesse.to_string();
    } 
}

/***
 * Fonction pour ajouter une légende
 */
fn setup_legend(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    let legend_texture_handle = asset_server.load("legende.png");

    if let Some(mut window) = windows.get_single_mut().ok() {
        commands.spawn(SpriteBundle {
            texture: legend_texture_handle,
            transform: Transform::from_xyz(-5.0, 30.0, 0.0).with_scale(Vec3::splat(0.01)),
            ..Default::default()
        });
    }
}

/***
 * Fonction pour activer ou désactiver le plein écran
 */
fn toggle_fullscreen(input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::F11) {
        for mut window in windows.iter_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen,
                _ => WindowMode::Windowed,
            };
        }
    }
}

/***
 * Fonction pour quitter le jeu
 */
fn toggle_exit_game(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    } 
}

/***
 * Fonction pour demander la résolution à l'utilisateur
 */
fn request_resolution_from_user() -> (f32, f32) {
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

fn main() {
    let seed_option = request_seed_from_user();
    let (width, height) = request_resolution_from_user();
    let nb_robots = request_nb_robots();

    App::new()
        .add_plugins(
            (DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Essaim de Robots pour Exploration et Etude Astrobiologique".to_string(),
                    mode: WindowMode::Windowed,
                    resolution: (width, height).into(),
                    ..default()
                }),
                ..default()
            })),
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
