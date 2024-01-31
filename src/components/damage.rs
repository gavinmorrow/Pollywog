use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bundles::player::Player;

use super::health::Health;

pub struct DamagePlugin;
impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_damages_player);
    }
}

#[derive(Component)]
pub struct Damage(pub f32);

// FIXME: buggy. try jumping on it very very quickly and it doesn't register.
pub fn enemy_damages_player(
    mut collisions: EventReader<CollisionEvent>,
    mut player: Query<(Entity, &mut Health), With<Player>>,
    kills_player_query: Query<(Entity, &Damage)>,
) {
    let (player_entity, mut player_health) = player.single_mut();

    for collision in collisions.read() {
        if let CollisionEvent::Started(entity_a, entity_b, _flags) = collision {
            let entities = [entity_a, entity_b];
            if entities.contains(&&player_entity)
            // && kills_player_query.iter().any(|e| entities.contains(&&e))
            {
                for (entity, damage) in kills_player_query.iter() {
                    if entities.contains(&&entity) {
                        // Collision with player
                        // Kill player
                        player_health.remaining -= damage.0;
                    }
                }
            }
        }
    }
}
