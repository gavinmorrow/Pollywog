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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    info!("Starting setup");

    let window = window_query.single();

    debug!("Spawning camera");
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(window.width() / 2.0 - 100.0, window.height() / 2.0, 1000.0),
            ..default()
        },
        ..default()
    });

    debug!("Spawning player");
    commands.spawn(player::PlayerBundle::new(asset_server, window));

    // Spawn the blocks in the level
    // FIXME: this should be loaded from a file
    debug!("Spawning blocks");
    let level = level::Level::<100>::new(0);
    for block in level.blocks {
        commands.spawn(block);
    }
}
