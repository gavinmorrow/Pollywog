use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::in_game::{bundles::enemy::Enemy, player::Player, InGameSet};

use super::health::Health;

pub fn damage_plugin(app: &mut App) {
    app.add_systems(Update, enemy_damage_player.in_set(InGameSet));
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
        for (enemy_entity, _enemy_char_controller, damage) in &enemies {
            if collision.entity == enemy_entity {
                damages.insert(enemy_entity, damage);
                trace!("player hit enemy, applying {:?} damage", damage);
            }
        }
    }

    // Iterate over all collisions with the enemies
    for (enemy_entity, enemy_char_controller, damage) in &enemies {
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
