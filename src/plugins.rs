use bevy::prelude::*;

pub mod dead_screen;
pub mod in_game;
pub mod start_screen;
pub mod win_screen;

pub fn screens_plugin(app: &mut App) {
    app.add_plugins((
        dead_screen::dead_screen_plugin,
        in_game::in_game_plugin,
        start_screen::start_screen_plugin,
        win_screen::win_screen_plugin,
    ));
}
