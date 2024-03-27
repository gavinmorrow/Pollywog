use bevy::prelude::*;

use crate::{bundles::player::Player, state::GameState};

pub fn player_win_plugin(app: &mut App) {
    app.add_systems(Update, check_win.run_if(in_state(GameState::InGame)))
        .add_systems(OnEnter(GameState::Win), on_win);
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

fn on_win() {
    bevy::log::info!("You won!");
}
