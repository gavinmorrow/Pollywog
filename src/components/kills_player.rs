use bevy::prelude::*;

use crate::{bundles::player::Player, state::GameState};

use super::health::Health;

pub fn kills_player_plugin(app: &mut App) {
    app.add_systems(
        Update,
        kills_player
            .after(super::damage::enemy_damage_player)
            .run_if(in_state(GameState::InGame)),
    );
}

pub fn kills_player(
    player_health: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_health.single().remaining <= 0.0 {
        next_state.set(GameState::Dead);
    }
}
