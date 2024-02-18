use bevy::prelude::*;

use crate::{bundles::player::Player, state::GameState};

pub struct PlayerWinPlugin;
impl Plugin for PlayerWinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_win.run_if(state_exists_and_equals(GameState::InGame)),
        )
        .add_systems(OnEnter(GameState::Win), on_win);
    }
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
    panic!("You won!");
}
