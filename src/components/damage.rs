use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bundles::{enemy::Enemy, player::Player};

use super::health::Health;

pub struct DamagePlugin;
impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_damages_player);
    }
}

#[derive(Component)]
pub struct Damage(pub f32);

pub fn enemy_damages_player(
    mut player: Query<(&KinematicCharacterControllerOutput, &mut Health), With<Player>>,
    enemies: Query<(Entity, &Damage), With<Enemy>>,
) {
    let Ok((player_char_controller, mut player_health)) = player.get_single_mut() else {
        // FIXME: since if the player didn't move, then the player_char_controller will be None.
        //        if the enemy collides with the player then, the collision won't be registered.
        return;
    };

    for collision in &player_char_controller.collisions {
        for (enemy_entity, damage) in enemies.iter() {
            if collision.entity == enemy_entity {
                player_health.remaining -= damage.0;
            }
        }
    }
}
