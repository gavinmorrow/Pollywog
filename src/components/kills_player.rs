use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{bundles::player::Player, state::GameState};

use super::health::Health;

pub struct KillsPlayerPlugin;
impl Plugin for KillsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (enemy_damages_player, kills_player, add_active_events),
        );
    }
}

#[derive(Component)]
pub struct KillsPlayerComponent;

// FIXME: buggy. try jumping on it very very quickly and it doesn't register.
pub fn enemy_damages_player(
    mut collisions: EventReader<CollisionEvent>,
    mut player: Query<(Entity, &mut Health), With<Player>>,
    kills_player_query: Query<Entity, With<KillsPlayerComponent>>,
) {
    let (player_entity, mut player_health) = player.single_mut();

    for collision in collisions.read() {
        if let CollisionEvent::Started(entity_a, entity_b, _flags) = collision {
            let entities = [entity_a, entity_b];
            if entities.contains(&&player_entity)
                && kills_player_query.iter().any(|e| entities.contains(&&e))
            {
                // Collision with player
                // Kill player
                player_health.remaining = 0.0;
            }
        }
    }
}

pub fn kills_player(
    player: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player.single().remaining <= 0.0 {
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
