use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{bundles::player::Player, state::GameState};

use super::health::Health;

pub struct KillsPlayerPlugin;
impl Plugin for KillsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (kills_player, add_active_events));
    }
}

pub fn kills_player(
    player_health: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_health.single().remaining <= 0.0 {
        next_state.set(GameState::Dead);
    }
}

fn add_active_events(mut active_events: Query<&mut ActiveEvents>) {
    // println!("start");
    for mut active_event in active_events.iter_mut() {
        // println!("modifying active event");
        *active_event = ActiveEvents::COLLISION_EVENTS;
    }
}
