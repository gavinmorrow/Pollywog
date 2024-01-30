use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player::Player, state::GameState};

pub struct KillsPlayerPlugin;
impl Plugin for KillsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (kills_player, add_active_events));
    }
}

#[derive(Component)]
pub struct KillsPlayerComponent;

// FIXME: buggy. try jumping on it very very quickly and it doesn't register.
pub fn kills_player(
    mut collisions: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    kills_player_query: Query<Entity, With<KillsPlayerComponent>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let player = &player_query.single();

    for collision in collisions.read() {
        eprintln!(
            "{:#?} {:#?} {:#?}",
            collision,
            player,
            kills_player_query.iter().next().unwrap()
        );
        if let CollisionEvent::Started(entity_a, entity_b, _flags) = collision {
            let entities = [entity_a, entity_b];
            if entities.contains(&player)
                && kills_player_query.iter().any(|e| entities.contains(&&e))
            {
                // Collision with player
                // Kill player
                next_state.set(GameState::Dead)
            }
        }
    }
}

fn add_active_events(mut active_events: Query<&mut ActiveEvents>) {
    // println!("start");
    for mut active_event in active_events.iter_mut() {
        // println!("modifying active event");
        *active_event = ActiveEvents::COLLISION_EVENTS;
    }
}
