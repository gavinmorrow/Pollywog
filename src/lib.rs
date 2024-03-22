use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*, window::WindowMode};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::state::GameState;

mod bundles;
mod camera;
mod components;
mod level;
mod screens;
mod state;

const PIXELS_PER_METER: f32 = 64.0;
const GRAVITY: Vec2 = Vec2::new(0.0, -9.81);

const BACKGROUND_COLOR: Color = Color::Rgba {
    red: 0.18,
    green: 0.21,
    blue: 0.20,
    alpha: 1.0,
};

pub fn start_app() {
    eprintln!("Creating app...");

    App::new()
        .init_state::<GameState>()
        .insert_resource(RapierConfiguration {
            gravity: GRAVITY,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((
            setup_default_plugins(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER),
            RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<components::character::Action>::default(),
            camera::camera_plugin,
            screens::screens_plugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,pollywog=debug".into(),
            level: bevy::log::Level::TRACE,
            ..default()
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
