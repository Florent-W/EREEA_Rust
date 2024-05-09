use bevy::{input::mouse::MouseWheel, prelude::*, window::WindowMode};

/***
 * Fonction pour la caméra
 */
pub fn setup_camera(mut commands: Commands, center_x: f32, center_y: f32) {
    let zoom_level = 0.05;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(center_x, center_y, 10.0)
            .with_scale(Vec3::new(zoom_level, zoom_level, 1.0)),
        ..default()
    });
    commands.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)));
}

/***
 * Fonction pour déplacer la caméra avec les touches directionnelles
 */
pub fn move_camera_system(
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
 * Fonction pour faire un zoom avec la caméra
 */
pub fn zoom_camera_system(
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
 * Fonction pour activer ou désactiver le plein écran
 */
pub fn toggle_fullscreen(input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::F11) {
        for mut window in windows.iter_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen,
                _ => WindowMode::Windowed,
            };
        }
    }
}
