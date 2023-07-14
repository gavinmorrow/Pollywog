use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*, window::PrimaryWindow};

mod gravity;
mod level;
mod player;
mod velocity;

pub fn start_app() {
    eprintln!("Creating app...");

    App::new()
        .add_plugins(setup_default_plugins())
        // Physics is run seperatly from the main loop.
        // It runs on a fixed time step to ensure that the physics is consistent.
        .insert_resource(FixedTime::new_from_secs(1.0 / 240.0))
        .add_systems(
            Startup,
            (spawn_camera, player::spawn, level::spawn_blocks::<100>),
        )
        .add_systems(
            FixedUpdate,
            (
                gravity::apply_gravity.before(velocity::update_velocity),
                velocity::update_velocity,
            ),
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

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    debug!("Spawning camera");
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(window.width() / 2.0 - 100.0, window.height() / 2.0, 1000.0),
            ..default()
        },
        ..default()
    });
}
