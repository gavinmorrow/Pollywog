use bevy::prelude::*;

use crate::{plugins::in_game::player::Player, state::GameState};

const MAX_SPEED_X: f32 = 3.0;
const MAX_SPEED_Y: f32 = 1.0;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera).add_systems(
        Update,
        keep_player_in_view
            .after(crate::components::character::r#move)
            .run_if(in_state(GameState::InGame)),
    );
}

pub fn spawn_camera(mut commands: Commands) {
    debug!("Spawning camera");
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1000.0),
            ..default()
        },
        ..default()
    });
}

pub fn keep_player_in_view(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    // The `Without<Camera>` allows us to have seperate queries for the player
    // and the camera (needed b/c they are both accessing `Transform`, and if
    // there somehow was a Transform w/ both `Player` and `Camera` components,
    // it wouldn't be allowed. we know that it isn't possible, but bevy doesn't)
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera = camera_query.single_mut();
    let Ok(player) = player_query.get_single() else {
        return;
    };

    follow(player.translation.x, &mut camera.translation.x, MAX_SPEED_X);
    follow(player.translation.y, &mut camera.translation.y, MAX_SPEED_Y);
}

fn follow(a: f32, b: &mut f32, max_speed: f32) {
    let delta = *b - a;
    let delta = delta / 100.0;
    let delta = if delta.abs() < 0.25 { 0.0 } else { delta };
    *b -= delta.min(max_speed);
}
