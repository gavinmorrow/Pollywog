use bevy::prelude::*;

use crate::bundles::enemy::{add_gravity, enemy_sprite_flipped, move_enemy, swap_direction};

pub struct NpcMovementPlugin;
impl Plugin for NpcMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_enemy,
                swap_direction,
                enemy_sprite_flipped,
                add_gravity,
            ),
        );
    }
}
