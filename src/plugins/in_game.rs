use bevy::prelude::*;

use crate::state::GameState;

mod bundles;
pub mod components;
pub mod level;
pub mod player;

pub fn in_game_plugin(app: &mut App) {
    let in_game_set_config = || InGameSet.run_if(in_state(GameState::InGame));

    app.add_plugins((
        player::player_plugin,
        level::level_plugin,
        components::ComponentsPlugin,
    ))
    .configure_sets(Update, in_game_set_config())
    .configure_sets(FixedUpdate, in_game_set_config())
    // Exit after `Win`/`Dead` so there can be a screenshot of the game in the background
    .add_systems(OnExit(GameState::Win), cleanup)
    .add_systems(OnExit(GameState::Dead), cleanup);
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InGameSet;

// TODO: clean this up so it doesn't need to import `components::character::grapple::GrappleState`
fn cleanup(
    commands: Commands,
    query: Query<Entity, With<level::LevelEntity>>,
    next_grapple_state: ResMut<NextState<components::character::grapple::GrappleState>>,
) {
    components::cleanup(next_grapple_state);
    level::despawn_entities(commands, query);
}
