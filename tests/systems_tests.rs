#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::app::App;
    use bevy::render::RenderPlugin;
    use ereea::{components::{setup_map, AffichageCasesNonDecouvertes, SeedResource}, systems::{setup_ui, toggle_cases_non_decouvertes}};

#[test]
fn test_camera_initialization() {
    let mut app = App::new();

        app.insert_resource(SeedResource { seed: Some(1) });
        app.insert_resource(Events::<AssetEvent<Image>>::default());
        app.add_plugins(MinimalPlugins);
        app.add_plugins(WindowPlugin::default());
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(RenderPlugin::default());
        app.update();

        app.add_systems(Startup, setup_map);
        app.update(); 

    // On regarde il y a combien d'entité qui ont des composants camera
    let camera_count = app.world.query::<&Camera2d>().iter(&app.world).count();
    println!("Number of cameras in the world: {}", camera_count);

    let mut query = app.world.query::<(&Transform, &Camera)>();
    let camera_transform = query.iter(&app.world).next().expect("Au moins une entité camera devrait être présente");

   // assert_eq!(camera_transform.0.translation.x, 0.0, "Camera x position should be 0.0");
   // assert!((camera_transform.0.scale.x - 0.05).abs() < f32::EPSILON, "Camera x scale should be 0.05");
}

#[test]
fn test_setup_ui() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.init_asset::<Font>();

    app.add_systems(Startup, setup_ui);

    app.update();
      // Check if UI entities with Text component are correctly created
      let mut text_query = app.world.query::<&Text>();
      assert!(text_query.iter(&app.world).count() > 0); 
}

#[test]
fn test_toggle_cases_non_decouvertes() {
    let mut app = App::new();
    app.insert_resource(AffichageCasesNonDecouvertes(false));
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.add_systems(Update, toggle_cases_non_decouvertes);
     //  .add_startup_system(setup_test_env.system());

    // Simuler une pression sur la touche Tab
    app.world.resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::Tab);
    app.update();

    // Vérifier si l'état a été basculé
    let affichage = app.world.resource::<AffichageCasesNonDecouvertes>();
    assert_eq!(affichage.0, true);  
    }
}

       
