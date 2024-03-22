use bevy::prelude::*;

mod dead_screen;
mod start_screen;

pub fn screens_plugin(app: &mut App) {
    dead_screen::dead_screen_plugin(app);
    start_screen::start_screen_plugin(app);
}
