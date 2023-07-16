use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*, window::WindowMode};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

mod camera;
mod level;
mod player;

pub fn start_app() {
    eprintln!("Creating app...");

    App::new()
        .add_plugins((
            setup_default_plugins(),
            PhysicsPlugins::default(),
            InputManagerPlugin::<player::Action>::default(),
            player::grapple::GrapplePlugin::default(),
        ))
        .insert_resource(level::Level::new(0))
        .insert_resource(Gravity(Vec2::new(0.0, -9.81 * 100.0)))
        .add_systems(
            Startup,
            (camera::spawn_camera, player::spawn, level::spawn_blocks),
        )
        .add_systems(
            Update,
            (
                camera::keep_player_in_view.after(player::r#move),
                player::r#move,
                bevy::window::close_on_esc,
            ),
        )
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
                mode: WindowMode::BorderlessFullscreen,
                title: "Pollywog".to_string(),
                ..default()
            }),
            ..default()
        })
}
