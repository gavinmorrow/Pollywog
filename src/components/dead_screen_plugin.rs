use bevy::prelude::*;

use crate::state::GameState;

pub struct DeadScreenPlugin;
impl Plugin for DeadScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Dead), on_dead);
    }
}

fn on_dead() {
    panic!("Player died. Sorry!")
}
