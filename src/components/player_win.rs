use bevy::prelude::*;

use crate::{plugins::in_game::player::Player, state::GameState};

pub fn player_win_plugin(app: &mut App) {
    app.add_systems(Update, check_win.run_if(in_state(GameState::InGame)));
}

fn check_win(
    player: Query<&Transform, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let x = player.single().translation.x;
    if x >= 64.0 * 32.0 {
        next_state.set(GameState::Win)
    }
}
