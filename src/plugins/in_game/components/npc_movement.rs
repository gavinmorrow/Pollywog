use bevy::prelude::*;

use crate::{
    bundles::enemy::{enemy_sprite_flipped, move_enemy, swap_direction},
    state::GameState,
};

pub fn npc_movement_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (move_enemy, swap_direction, enemy_sprite_flipped).run_if(in_state(GameState::InGame)),
    );
}
