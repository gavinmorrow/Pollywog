use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};
use bevy_xpbd_2d::prelude::*;

mod camera;
mod level;
mod player;

pub fn start_app() {
    eprintln!("Creating app...");

    App::new()
        .add_plugins((setup_default_plugins(), PhysicsPlugins::default()))
        .insert_resource(level::Level::new(0))
        .insert_resource(Gravity(Vec2::new(0.0, -9.81 * 100.0)))
        .add_systems(
            Startup,
            (camera::spawn_camera, player::spawn, level::spawn_blocks),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_default_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,pollywog=debug".into(),
        level: bevy::log::Level::TRACE,
    })
}
