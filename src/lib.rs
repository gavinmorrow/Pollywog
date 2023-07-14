use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};

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
        .add_systems(Startup, setup)
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Starting setup");

    commands.spawn(Camera2dBundle::default());
    commands.spawn(player::PlayerBundle::new(asset_server));
}
