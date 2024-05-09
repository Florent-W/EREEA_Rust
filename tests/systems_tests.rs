#[cfg(test)]
mod tests {
    use bevy::app::App;
    use bevy::prelude::*;
    use ereea::{
        components::AffichageCasesNonDecouvertes,
        systems::{setup_ui, toggle_cases_non_decouvertes},
    };

    /***
     * Test pour les systèmes de UI
     */
    #[test]
    fn test_setup_ui() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Font>();

        app.add_systems(Startup, setup_ui);

        app.update();

        let mut text_query = app.world.query::<&Text>();
        assert!(text_query.iter(&app.world).count() > 0);
    }

    /***
     * Test pour le système de basculement de l'affichage du mode découverte
     */
    #[test]
    fn test_toggle_cases_non_decouvertes() {
        let mut app = App::new();
        app.insert_resource(AffichageCasesNonDecouvertes(false));
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.add_systems(Update, toggle_cases_non_decouvertes);

        app.world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Tab);
        app.update();

        // Vérifier si l'état a été basculé
        let affichage = app.world.resource::<AffichageCasesNonDecouvertes>();
        assert!(affichage.0);
    }
}

/* TODO Finir les tests
/***
 * Test pour le système de déplacement de la caméra

 */
#[test]
fn test_camera_initialization() {
    let mut app = App::new();

        app.insert_resource(SeedResource { seed: Some(1) });
        app.add_plugins(MinimalPlugins);
        app.add_plugins(WindowPlugin::default());
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(RenderPlugin::default());
        app.update();

        app.add_systems(Startup, setup_map);
        app.update();

    // On regarde il y a combien d'entité qui ont des composants camera
    let camera_count = app.world.query::<&Camera2d>().iter(&app.world).count();
    println!("Nombre de caméra {}", camera_count);

    let mut query = app.world.query::<(&Transform, &Camera)>();
    let _camera_transform = query.iter(&app.world).next().expect("Au moins une entité camera devrait être présente");
}


#[cfg(test)]
mod tests1 {
    use super::*;
    use bevy::window::WindowMode;
    use ereea::systems::setup_camera;

    #[test]
    fn test_camera_initialization() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.add_startup_system(|mut commands: Commands| {
            setup_camera(&mut commands, 5.0, 5.0);
        });
        app.update();

        let query = app.world.query::<&Transform, With<Camera2d>>();
        let transform = query.single(&app.world).unwrap();

        assert_eq!(transform.translation, Vec3::new(5.0, 5.0, 10.0));
        assert_eq!(transform.scale, Vec3::new(0.05, 0.05, 1.0));
    }

    #[test]
    fn test_move_camera() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.insert_resource(Time::default());
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(Camera2dBundle::default());
        });
        app.add_system(move_camera_system.system());

        // Simuler l'appui sur la touche gauche
        app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::ArrowLeft);
        app.update();

        let transform = app.world.query::<&Transform, With<Camera2d>>().single(&app.world).unwrap();
        assert_eq!(transform.translation.x, -10.0 * app.world.resource::<Time>().delta_seconds());
    }

    #[test]
    fn test_zoom_camera() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(Camera2dBundle::default());
        });
        app.add_event::<MouseWheel>();
        app.add_system(zoom_camera_system.system());

        // Simuler le zoom avant
        app.world.send_event(MouseWheel { unit: MouseScrollUnit::Line, x: 0.0, y: 1.0 });
        app.update();

        let transform = app.world.query::<&Transform, With<Camera2d>>().single(&app.world).unwrap();
        assert!(transform.scale.x > 1.0); // Zoom avant augmente le scale
    }

    #[test]
    fn test_toggle_fullscreen() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(WindowMode::default());
        });
        app.add_system(toggle_fullscreen.system());

        app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F11);
        app.update();

        let window = app.world.query::<&Window>().single(&app.world).unwrap();
        assert_eq!(window.mode, WindowMode::BorderlessFullscreen);
    }
}



#[cfg(test)]
mod tests2 {
    use super::*;
    use bevy::prelude::*;
    use bevy::app::App;

    #[test]
    fn test_increase_speed() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins); // Inclure les plugins par défaut pour une gestion correcte des ressources.
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.insert_resource(VitesseGlobale { vitesse: 100 });
        app.add_startup_system(|mut commands: Commands| {
            commands.spawn().insert(Text::from("Speed: 100")).insert(TexteVitesse);
        });
        app.add_system(toggle_vitesse.system());

        // Simuler l'appui sur F2 pour augmenter la vitesse
        app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F2);
        app.update();

        let vitesse_globale = app.world.resource::<VitesseGlobale>().clone();
        let query = app.world.query::<&Text, With<TexteVitesse>>();
        let text = query.single(&app.world).unwrap();

        assert_eq!(vitesse_globale.vitesse, 101);
        assert_eq!(text.sections[0].value, "Speed: 101");
    }

    #[test]
    fn test_decrease_speed() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.insert_resource(VitesseGlobale { vitesse: 100 });
        app.add_startup_system(|mut commands: Commands| {
            commands.spawn().insert(Text::from("Speed: 100")).insert(TexteVitesse);
        });
        app.add_system(toggle_vitesse.system());

        // Simuler l'appui sur F1 pour diminuer la vitesse
        app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F1);
        app.update();

        let vitesse_globale = app.world.resource::<VitesseGlobale>().clone();
        let query = app.world.query::<&Text, With<TexteVitesse>>();
        let text = query.single(&app.world).unwrap();

        assert_eq!(vitesse_globale.vitesse, 99);
        assert_eq!(text.sections[0].value, "Speed: 99");
    }
}




#[cfg(test)]
mod tests3 {
    use super::*;
    use bevy::prelude::*;
    use bevy::app::App;
    use bevy::text::Text;
    use ereea::components::Compteur;
    use ereea::systems::update_text;

    #[test]
    fn test_ui_setup() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins); // Ajout des plugins par défaut pour gérer les systèmes de UI.
        app.add_startup_system(setup_ui.system());
        app.update();

        let query_energie = app.world.query::<&Text, With<TexteEnergie>>();
        let text_energie = query_energie.single(&app.world).unwrap();
        assert_eq!(text_energie.sections[0].value, "Énergies: ");
        assert_eq!(text_energie.sections[1].value, "0");

        let query_minerai = app.world.query::<&Text, With<TexteMinerai>>();
        let text_minerai = query_minerai.single(&app.world).unwrap();
        assert_eq!(text_minerai.sections[0].value, "Minerais: ");
        assert_eq!(text_minerai.sections[1].value, "0");

        let query_vitesse = app.world.query::<&Text, With<TexteVitesse>>();
        let text_vitesse = query_vitesse.single(&app.world).unwrap();
        assert_eq!(text_vitesse.sections[0].value, "Vitesse: x");
        assert_eq!(text_vitesse.sections[1].value, "1");
    }

    #[test]
    fn test_update_text() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins); // Assurer que tous les plugins nécessaires sont chargés
        app.add_startup_system(setup_ui.system());
        app.insert_resource(Compteur { energie: 100, minerai: 50 });
        app.add_system(update_text.system());
        app.update();

        let query_energie = app.world.query::<&Text, With<TexteEnergie>>();
        let text_energie = query_energie.single(&app.world).unwrap();
        assert_eq!(text_energie.sections[1].value, "100");

        let query_minerai = app.world.query::<&Text, With<TexteMinerai>>();
        let text_minerai = query_minerai.single(&app.world).unwrap();
        assert_eq!(text_minerai.sections[1].value, "50");
    }

    #[test]
    fn test_update_text_vitesse() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.add_startup_system(setup_ui.system());
        app.insert_resource(VitesseGlobale { vitesse: 5 });
        app.add_system(update_text_vitesse.system());
        app.update();

        let query_vitesse = app.world.query::<&Text, With<TexteVitesse>>();
        let text_vitesse = query_vitesse.single(&app.world).unwrap();
        assert_eq!(text_vitesse.sections[1].value, "5");
    }
}




#[cfg(test)]
mod tests4 {
    use super::*;
    use bevy::prelude::*;
    use bevy::app::App;
    use bevy::ecs::system::SystemState;

    #[test]
    fn test_toggle_bordures() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.insert_resource(BorduresActive(false));
        app.add_startup_system_to_stage(StartupStage::Startup, |mut commands: Commands| {
            commands.spawn().insert(Bordure).insert(Visibility::Hidden);
        });
        app.add_system(toggle_bordures.system());

        app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F3);
        app.update();

        let visibility = app.world.query::<&Visibility, With<Bordure>>().single(&app.world).unwrap();
        assert_eq!(visibility.is_visible(), true, "Les bordures devraient être visibles");

        app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F3);
        app.update();

        let visibility = app.world.query::<&Visibility, With<Bordure>>().single(&app.world).unwrap();
        assert_eq!(visibility.is_visible(), false, "Les bordures devraient être invisibles");
    }

    #[test]
    fn test_toggle_cases_non_decouvertes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.insert_resource(AffichageCasesNonDecouvertes(false));
        app.add_system(toggle_cases_non_decouvertes.system());

        app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::Tab);
        app.update();

        let affichage = app.world.resource::<AffichageCasesNonDecouvertes>().0;
        assert_eq!(affichage, true, "L'affichage devrait être activé");

        // Simuler une autre pression de la touche Tab pour désactiver l'affichage
        app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::Tab);
        app.update();

        let affichage = app.world.resource::<AffichageCasesNonDecouvertes>().0;
        assert_eq!(affichage, false, "L'affichage devrait être désactivé");
    }

    #[test]
    fn test_toggle_exit_game() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.add_event::<AppExit>();
        app.add_system(toggle_exit_game.system());

        app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::Escape);
        app.update();

        let exit_events = app.world.resource::<Events<AppExit>>();
        let mut reader = exit_events.get_reader();
        assert!(reader.iter(&exit_events).next().is_some(), "Un événement de sortie aurait dû être déclenché");
    }
}
*/
