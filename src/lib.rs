use bevy::{log::LogPlugin, prelude::*};

mod player;

pub fn start_app() {
    eprintln!("Creating app...");

    let default_plugins = DefaultPlugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,pollywog=trace".into(),
        level: bevy::log::Level::TRACE,
    });

    App::new()
        .add_plugins(default_plugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Starting setup");

    commands.spawn(Camera2dBundle::default());
    commands.spawn(player::PlayerBundle::new(asset_server));
}
