use bevy::prelude::*;

mod dead_screen;
mod in_game;
mod start_screen;

pub fn screens_plugin(app: &mut App) {
    app.add_plugins((
        dead_screen::dead_screen_plugin,
        in_game::in_game_plugin,
        start_screen::start_screen_plugin,
    ));
}
