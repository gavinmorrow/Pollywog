use bevy::{prelude::*, window::PrimaryWindow};

/// The starting X position of the camera (from the left edge of the window).
const START_X: f32 = 100.0;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    debug!("Spawning camera");
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(
                window.width() / 2.0 - START_X,
                window.height() / 2.0,
                1000.0,
            ),
            ..default()
        },
        ..default()
    });
}
