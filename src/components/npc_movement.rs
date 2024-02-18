use bevy::prelude::*;

use crate::{
    bundles::enemy::{enemy_sprite_flipped, move_enemy, swap_direction},
    state::GameState,
};

pub struct NpcMovementPlugin;
impl Plugin for NpcMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_enemy, swap_direction, enemy_sprite_flipped)
                .run_if(state_exists_and_equals(GameState::InGame)),
        );
    }
}
