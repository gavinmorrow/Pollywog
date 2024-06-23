use bevy::prelude::*;

use crate::{bundles, components, state::GameState};

pub mod level;

pub fn in_game_plugin(app: &mut App) {
    app.add_plugins((
        bundles::player::player_plugin,
        level::level_plugin,
        components::ComponentsPlugin,
    ))
    // Exit after `Dead` so there can be a screenshot of the game in the background
    .add_systems(OnExit(GameState::Dead), cleanup);
}

// TODO: clean this up so it doesn't need to import `components::character::grapple::GrappleState`
fn cleanup(
    commands: Commands,
    query: Query<Entity, With<level::LevelEntity>>,
    next_grapple_state: ResMut<NextState<components::character::grapple::GrappleState>>,
) {
    components::cleanup(next_grapple_state);
    level::despawn_entities(commands, query);
}
