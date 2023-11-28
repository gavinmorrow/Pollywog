use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*, window::WindowMode};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

mod camera;
mod level;
mod player;

const PIXELS_PER_METER: f32 = 100.0;

pub fn start_app() {
    eprintln!("Creating app...");

    App::new()
        .add_plugins((
            setup_default_plugins(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER),
            InputManagerPlugin::<player::Action>::default(),
            camera::CameraPlugin,
            player::PlayerPlugin,
            level::LevelPlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,pollywog=trace".into(),
            level: bevy::log::Level::TRACE,
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                title: "Pollywog".to_string(),
                ..default()
            }),
            ..default()
        })
}
