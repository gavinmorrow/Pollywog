use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    bundles::{enemy::Enemy, player::Player},
    state::GameState,
};

use super::health::Health;

pub struct DamagePlugin;
impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            enemy_damage_player.run_if(state_exists_and_equals(GameState::InGame)),
        );
    }
}

#[derive(Component, Debug)]
pub struct Damage(pub f32);

pub fn enemy_damage_player(
    mut player: Query<(Entity, &KinematicCharacterControllerOutput, &mut Health), With<Player>>,
    enemies: Query<(Entity, &KinematicCharacterControllerOutput, &Damage), With<Enemy>>,
) {
    let Ok((player_entity, player_char_controller, mut player_health)) = player.get_single_mut()
    else {
        // Player not created yet
        trace!("Player not created yet. Skipping damage system.");
        return;
    };

    let mut damages = HashMap::new();

    // Iterate over all collisions with the player
    for collision in &player_char_controller.collisions {
        for (enemy_entity, _enemy_char_controller, damage) in enemies.iter() {
            if collision.entity == enemy_entity {
                damages.insert(enemy_entity, damage);
                trace!("player hit enemy, applying {:?} damage", damage);
            }
        }
    }

    // Iterate over all collisions with the enemies
    for (enemy_entity, enemy_char_controller, damage) in enemies.iter() {
        for collision in &enemy_char_controller.collisions {
            if collision.entity == player_entity {
                damages.insert(enemy_entity, damage);
                trace!("enemy hit player, applying {:?} damage", damage);
            }
        }
    }

    // Apply the damages
    for damage in damages.values() {
        player_health.remaining -= damage.0;
    }
}
