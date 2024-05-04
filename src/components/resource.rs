use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct AffichageCasesNonDecouvertes(pub bool);

#[derive(Resource, Debug)]
pub struct BorduresActive(pub bool);

#[derive(Resource, Debug)]
pub struct SeedResource {
    pub seed: Option<u32>,
}

#[derive(Resource, Debug)]
pub struct SizeMap {
    pub length: Option<u32>,
    pub height: Option<u32>
}

#[derive(Resource, Debug)]
pub struct Compteur {
   pub minerai: u32,
   pub energie: u32,
   pub total_robots: u32,
}

#[derive(Resource, Debug)]
pub struct VitesseGlobale {
    pub vitesse: u32
}

#[derive(Component, Debug, PartialEq)]
pub enum Ressource {
    Energie,
    Mineral,
    LieuInteretScientifique,
    Obstacle,
}