use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    debug!("Spawning camera");
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(window.width() / 2.0 - 100.0, window.height() / 2.0, 1000.0),
            ..default()
        },
        ..default()
    });
}
