use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::Player;

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

pub fn keep_player_in_view(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // Using the tuple instead of `With<Player>` allows us to have seperate queries
    // for the player and the camera (needed b/c they are both accessing `Transform`).
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let window = window_query.single();
    let mut camera = camera_query.single_mut();
    let Ok(player) = player_query.get_single() else { return; };

    let player_x = player.translation.x;
    let camera_x = camera.translation.x;

    const PADDING: f32 = 256.0;

    let distance = camera_x - player_x;
    if distance.abs() > window.width() / 2.0 - PADDING {
        // move the camera to catch up with the player
        camera.translation.x = player_x + (window.width() / 2.0 - PADDING) * distance.signum();
    }
}
