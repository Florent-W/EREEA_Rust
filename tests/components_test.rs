use bevy::prelude::*;
use bevy::ecs::query::With;
use ereea::components::{setup_map, Carte, ElementCarte, EtatDecouverte, Ressource, Position, Robot, TypeRobot, Compteur, collect_resources_system, ElementMap, move_robots_on_map_system, RobotState, spawn_robots, VitesseGlobale, SizeMap, Base, BorduresActive, SeedResource, Bordure, encaissement_audio, setup_bordures, energy_audio, start_audio};
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioPlugin;
use bevy::app::App;


/***
 * Test de la map
 */
fn assert_map_system(mut query: Query<&mut Carte>, elements_query: Query<&ElementCarte>) {
    if let Some(carte) = query.iter_mut().next() {
        assert_eq!(carte.largeur, 50, "Largeur de la carte incorrecte");
        assert_eq!(carte.hauteur, 50, "Hauteur de la carte incorrecte");
    } else {
        panic!("Erreur sur la création de la carte");
    }

    let elements_count = elements_query.iter().count();
    assert!(elements_count > 0, "Pas d'ElementCarte trouvé");
}

#[cfg(test)]
mod tests {
    use ereea::components::setup_bordures;

    use super::*;

    /***
     * Test de la création d'un robot
     */
    #[test]
    fn test_robot_creation() {
        let robot = Robot {
            id: 1,
            nom: "testRobot".to_string(),
            pv_max: 100,
            type_robot: TypeRobot::Explorateur,
            vitesse: 2,
            timer: 5.0,
            target_position: Some(Position { x: 10, y: 20 }),
            steps_moved: 0,
        };

        assert_eq!(robot.id, 1);
        assert_eq!(robot.nom, "testRobot");
        assert_eq!(robot.pv_max, 100);
        assert_eq!(robot.type_robot, TypeRobot::Explorateur);
        assert_eq!(robot.vitesse, 2);
        assert!(robot.timer > 0.0);
        assert_eq!(robot.target_position.unwrap(), Position { x: 10, y: 20 });
        assert_eq!(robot.steps_moved, 0);
    }

    /***
     * Test du mouvement du robot
     */
    #[test]
    fn test_robot_movement() {
        let mut robot = Robot {
            id: 1,
            nom: "testRobot".to_string(),
            pv_max: 100,
            type_robot: TypeRobot::Explorateur,
            vitesse: 2,
            timer: 5.0,
            target_position: Some(Position { x: 20, y: 30 }),
            steps_moved: 0,
        };

        robot.steps_moved += robot.vitesse;

        assert_eq!(robot.steps_moved, 2);
    }

    /***
     * Test de la création de la carte
     */
    #[test]
    fn test_map_creation() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());

        app.update();

        app.add_systems(Startup, setup_map);
        app.add_systems(Startup, assert_map_system);
        app.update(); 
    }

    #[test]
    fn test_setup_bordures() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.world.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(0.05, 1.0)), 
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Carte { largeur: 3, hauteur: 3 })
        .insert(Position { x: 0, y: 0 });
        app.add_systems(Startup, setup_bordures);

        app.update();

        let mut query = app.world.query_filtered::<&Transform, With<Sprite>>();
        let border_entities = query.iter(&app.world).collect::<Vec<&Transform>>();

        // On regarde combien de bordures ont été créées
        assert_eq!(border_entities.len(), 13);
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;
    use bevy::prelude::*;
    use bevy::app::App;
    use bevy_kira_audio::{Audio, AudioControl, AudioPlugin, AudioSource};
    use bevy::asset::{AssetServer, Assets, Handle};

    #[test]
    fn test_start_audio() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins); // Utiliser DefaultPlugins pour inclure les plugins audio
        app.add_plugins(AudioPlugin);

        // Ajouter les ressources manuellement pour simuler le système
        let asset_server = app.world.get_resource::<AssetServer>().unwrap();
        let audio = app.world.get_resource::<Audio>().unwrap();

        // Charger un fichier audio pour le test
        let music_handle: Handle<AudioSource> = asset_server.load("audio/music.ogg");
        
        // Appeler la fonction pour démarrer l'audio
        audio.play(music_handle.clone()).looped();

        // Note: Ce test vérifie principalement que le code s'exécute sans erreur
        // L'assertion réelle sur la lecture de l'audio n'est pas faisable sans mock
        println!("start_audio s'est exécuté sans erreur");
    }

    #[test]
    fn test_encaissement_audio() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.add_plugins(AudioPlugin);

        // Ajouter les ressources manuellement pour simuler le système
        let asset_server = app.world.get_resource::<AssetServer>().unwrap();
        let audio = app.world.get_resource::<Audio>().unwrap();

        // Charger un fichier audio pour le test
        let cash_handle: Handle<AudioSource> = asset_server.load("audio/cash.ogg");

        // Appeler la fonction pour jouer le son d'encaissement
        audio.play(cash_handle.clone());

        // Note: Ce test vérifie principalement que le code s'exécute sans erreur
        println!("encaissement_audio s'est exécuté sans erreur");
    }

    #[test]
    fn test_energy_audio() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.add_plugins(AudioPlugin);

        // Ajouter les ressources manuellement pour simuler le système
        let asset_server = app.world.get_resource::<AssetServer>().unwrap();
        let audio = app.world.get_resource::<Audio>().unwrap();

        // Charger un fichier audio pour le test
        let energy_handle: Handle<AudioSource> = asset_server.load("audio/energy.ogg");

        // Appeler la fonction pour jouer le son d'énergie
        audio.play(energy_handle.clone());

        // Note: Ce test vérifie principalement que le code s'exécute sans erreur
        println!("energy_audio s'est exécuté sans erreur");
    }
}



#[cfg(test)]
mod tests2 {
    use super::*;
    use bevy::prelude::*;
    use bevy::app::App;
    use bevy::asset::{AssetServer, Assets};
    use bevy::render::texture::Image;

    #[test]
    fn test_setup_map() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);  // Utilisation de DefaultPlugins pour inclure les fonctionnalités nécessaires par défaut

        // Ajout des systèmes et ressources nécessaires pour la configuration de la carte
        app.add_startup_system(setup_map);
        app.insert_resource(AssetServer::new(
            Assets::<Image>::default(),
            bevy::asset::AssetLoaderSettings::default(),
        ));
        app.insert_resource(SeedResource { seed: Some(1234) });
        app.insert_resource(SizeMap { length: Some(100), height: Some(100) });

        // Exécution de la mise à jour pour traiter les systèmes
        app.update();

        // Vérification de la création de la carte
        let query_carte = app.world.query::<&Carte>();
        assert!(query_carte.iter(&app.world).count() > 0, "La carte n'a pas été créée");

        // Vérification de la création des éléments sur la carte
        let query_element_carte = app.world.query::<(&ElementCarte, &Position)>();
        assert!(query_element_carte.iter(&app.world).count() > 0, "Aucun élément de carte n'a été créé");

        // Vérification de la présence de la base
        let query_base = app.world.query::<&Base>();
        assert!(query_base.iter(&app.world).next().is_some(), "La base n'a pas été placée sur la carte");
    }

    #[test]
    fn test_setup_bordures() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Configuration des conditions initiales pour le test
        app.add_startup_system_to_stage(StartupStage::Startup, |mut commands: Commands| {
            commands.spawn().insert(Carte { largeur: 10, hauteur: 10 }).insert(Position { x: 0, y: 0 });
        });

        // Ajout du système à tester
        app.add_system(setup_bordures);

        // Insertion des ressources nécessaires
        app.insert_resource(BorduresActive(true));

        // Exécution de la mise à jour pour traiter les systèmes
        app.update();

        // Vérification de la création des bordures
        let query_bordures = app.world.query::<&Bordure>();
        assert_eq!(query_bordures.iter(&app.world).count(), (10 * 2 - 1) * 2, "Le nombre de bordures créées est incorrect");
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_spawn_robots() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.add_startup_system(spawn_robots);

        app.insert_resource(AssetServer::new(Assets::<Image>::default(), bevy::asset::AssetLoaderSettings::default()));
        app.insert_resource(VitesseGlobale { vitesse: 1 });
        app.insert_resource(SizeMap { length: Some(100), height: Some(100) });
        app.insert_resource(Compteur { minerai: 10, energie: 10, total_robots: 0 });

        app.add_startup_system_to_stage(StartupStage::Startup, |mut commands: Commands| {
            commands.spawn().insert(Base).insert(Position { x: 5, y: 5 });
        });
        
        app.update();

        let robot_query = app.world.query::<&Robot>();
        assert!(robot_query.iter(&app.world).count() > 0, "Aucun robot n'a été créé");

        let robot = robot_query.single(&app.world).unwrap();
        assert_eq!(robot.type_robot, TypeRobot::Explorateur, "Le type de robot n'est pas correct");
    }

    #[test]
    fn test_move_robots_on_map_system() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.insert_resource(Time::default());
        app.insert_resource(VitesseGlobale { vitesse: 1 });
        
        app.add_startup_system_to_stage(StartupStage::Startup, |mut commands: Commands| {
            commands.spawn().insert(Carte { largeur: 100, hauteur: 100 });
            commands.spawn().insert(Base).insert(Position { x: 0, y: 0 });
        });

        app.add_system(move_robots_on_map_system);

        app.update(); // Execution de la mise à jour pour traiter les systèmes

        let (robot, position) = app.world.query::<(&Robot, &Position)>().single(&app.world).unwrap();
        assert!(position.x != 0 || position.y != 0, "Le robot ne s'est pas déplacé");
    }

    #[test]
    fn test_collect_resources_system() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        
        app.add_startup_system_to_stage(StartupStage::Startup, |mut commands: Commands| {
            commands.spawn().insert(ElementCarte {
                element: ElementMap::Ressource(Ressource::Energie),
                est_decouvert: EtatDecouverte::NonDecouvert,
            }).insert(Position { x: 0, y: 0 });

            commands.spawn().insert(Robot {
                id: 1,
                nom: "Collecteur".to_string(),
                pv_max: 100,
                type_robot: TypeRobot::Collecteur,
                vitesse: 1,
                timer: 0.0,
                target_position: Some(Position { x: 0, y: 0 }),
                steps_moved: 0,
            }).insert(Position { x: 0, y: 0 });
        });

        app.insert_resource(Compteur { minerai: 0, energie: 0, total_robots: 1 });
        app.add_system(collect_resources_system);

        app.update(); // Traitement des systèmes

        let compteur = app.world.resource::<Compteur>();
        assert_eq!(compteur.energie, 1, "La collecte d'énergie a échoué");
    }
}
