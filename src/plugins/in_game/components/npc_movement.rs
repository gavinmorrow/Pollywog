use bevy::prelude::*;

use crate::plugins::in_game::{
    bundles::enemy::{enemy_sprite_flipped, move_enemy, swap_direction},
    InGameSet,
};

pub fn npc_movement_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (move_enemy, swap_direction, enemy_sprite_flipped).in_set(InGameSet),
    );
}
