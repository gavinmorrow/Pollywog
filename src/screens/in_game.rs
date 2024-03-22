use bevy::prelude::*;

use crate::{bundles, components, level};

pub fn in_game_plugin(app: &mut App) {
    app.add_plugins((
        bundles::player::player_plugin,
        level::level_plugin,
        components::ComponentsPlugin,
    ));
}
