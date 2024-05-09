use crate::components::SeedResource;
use crate::systems::camera::setup_camera;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use super::{BorduresActive, Ressource, SizeMap};

const ENERGIE_SPRITE: &str = "textures/energie.png";
const MINERAL_SPRITE: &str = "textures/minerai.png";
const LIEU_INTERET_SPRITE: &str = "textures/lieu.png";
const BASE_SPRITE: &str = "textures/base.png";
const OBSTACLE_SPRITE: &str = "textures/obstacle.png";

#[derive(Component, Debug)]
pub struct Carte {
    pub largeur: u32,
    pub hauteur: u32,
}

#[derive(Component, PartialEq, Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Bordure;

#[derive(Component, Debug)]
pub struct ElementCarte {
    pub element: ElementMap,
    pub est_decouvert: EtatDecouverte,
    pub decouvert_robot_id: Option<u32>,
}

#[derive(Component, Debug)]
pub struct Base;

#[derive(Component, PartialEq, Debug)]
pub enum ElementMap {
    Ressource(Ressource),
    ElementGeographique(ElementGeographique),
}

#[derive(Component, Debug, PartialEq)]
pub enum ElementGeographique {
    Herbe,
    Terre,
    Eau,
    Sable,
    Montagne,
}

#[derive(Component, Debug, PartialEq)]
pub enum EtatDecouverte {
    NonDecouvert,
    EnAttente,
    Decouvert,
}

#[derive(Clone)]
enum TextureOrColor {
    Texture(Handle<Image>),
    Color(Color),
}

/***
 * Fonction pour charger la map
 */
pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    seed_res: Res<SeedResource>,
    size_map_res: Res<SizeMap>,
) {
    // Charger les textures pour les différents éléments de la carte
    let energie_texture_handle = asset_server.load(ENERGIE_SPRITE);
    let mineral_texture_handle = asset_server.load(MINERAL_SPRITE);
    let lieu_interet_texture_handle = asset_server.load(LIEU_INTERET_SPRITE);
    let base_handle = asset_server.load(BASE_SPRITE);
    let obstacle_handle = asset_server.load(OBSTACLE_SPRITE);

    // Dimensions de la carte
    let largeur = size_map_res.length.unwrap_or(50);
    let hauteur = size_map_res.height.unwrap_or(50);

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

            // Déterminer quel élément géographique générer en fonction de la valeur du bruit
            let (element_geo, texture_or_color_geo, taille_geo) = if noise_normalised >= 0.8 {
                (
                    ElementMap::ElementGeographique(ElementGeographique::Montagne),
                    TextureOrColor::Color(Color::rgb(0.8, 0.8, 0.8)),
                    1.0,
                )
            } else if noise_normalised >= 0.6 {
                (
                    ElementMap::ElementGeographique(ElementGeographique::Herbe),
                    TextureOrColor::Color(Color::rgb(0.5, 0.75, 0.3)),
                    1.0,
                )
            } else if noise_normalised > 0.4 {
                (
                    ElementMap::ElementGeographique(ElementGeographique::Terre),
                    TextureOrColor::Color(Color::rgb(0.69, 0.62, 0.541)),
                    1.0,
                )
            } else if noise_normalised > 0.2 {
                (
                    ElementMap::ElementGeographique(ElementGeographique::Sable),
                    TextureOrColor::Color(Color::rgb(0.76, 0.69, 0.5)),
                    1.0,
                )
            } else {
                (
                    ElementMap::ElementGeographique(ElementGeographique::Eau),
                    TextureOrColor::Color(Color::rgb(0.4, 0.5, 0.8)),
                    1.0,
                )
            };

            let sprite_bundle_geo = SpriteBundle {
                sprite: Sprite {
                    color: match texture_or_color_geo {
                        TextureOrColor::Color(color) => color,
                        _ => Color::WHITE,
                    },
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0))
                    .with_scale(Vec3::splat(taille_geo)),
                ..Default::default()
            };

            commands
                .spawn(sprite_bundle_geo)
                .insert(ElementCarte {
                    element: element_geo,
                    est_decouvert: EtatDecouverte::NonDecouvert,
                    decouvert_robot_id: None,
                })
                .insert(position);

            // Ajout des ressources
            if let Some((element_res, texture_or_color_res, taille_res)) = match noise_normalised {
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
                _ => None,
            } {
                let sprite_bundle_res = SpriteBundle {
                    texture: match texture_or_color_res {
                        TextureOrColor::Texture(texture_handle) => texture_handle,
                        _ => continue,
                    },
                    transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.1))
                        .with_scale(Vec3::splat(taille_res)),
                    ..Default::default()
                };

                commands
                    .spawn(sprite_bundle_res)
                    .insert(ElementCarte {
                        element: element_res,
                        est_decouvert: EtatDecouverte::NonDecouvert,
                        decouvert_robot_id: None,
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
        if (0.2..=0.5).contains(&noise_normalised) {
            break;
        }
    }

    // Place la base à la position valide trouvée
    commands
        .spawn(SpriteBundle {
            texture: base_handle,
            transform: Transform::from_translation(Vec3::new(base_x as f32, base_y as f32, 0.9))
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
pub fn setup_bordures(
    mut commands: Commands,
    query: Query<(&Carte, &Position)>,
    bordures_active: ResMut<BorduresActive>,
) {
    if !bordures_active.0 {
        return;
    }

    for (carte, carte_position) in query.iter() {
        let bordure_couleur = Color::BLACK;
        let epaisseur_bordure = 0.05;
        let taille_case = 1.0;
        for y in 0..carte.hauteur {
            for x in 0..carte.largeur {
                let x_pos = x as f32 + carte_position.x as f32 * taille_case;
                let y_pos = y as f32 + carte_position.y as f32 * taille_case;

                // Créer les bordures verticales
                if x < carte.largeur - 1 {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: bordure_couleur,
                                custom_size: Some(Vec2::new(epaisseur_bordure, taille_case)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(x_pos + 0.5 * taille_case, y_pos, 2.0),
                            visibility: Visibility::Visible,
                            ..Default::default()
                        })
                        .insert(Bordure);
                }

                // Créer les bordures horizontales
                if y < carte.hauteur - 1 {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: bordure_couleur,
                                custom_size: Some(Vec2::new(taille_case, epaisseur_bordure)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(x_pos, y_pos + 0.5 * taille_case, 2.0),
                            visibility: Visibility::Visible,
                            ..Default::default()
                        })
                        .insert(Bordure);
                }
            }
        }
    }
}
