use bevy::prelude::*;
use ereea::components::{Position, Robot, TypeRobot};
use ereea::components::{setup_map, Carte, ElementCarte};
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