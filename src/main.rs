use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    log::LogPlugin,
    prelude::*,
    window::WindowMode,
};
use bevy_rapier2d::prelude::*;
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};
use leafwing_input_manager::prelude::*;

use crate::state::GameState;

mod bundles;
mod camera;
mod components;
mod plugins;
mod state;

const PIXELS_PER_METER: f32 = 1.0;
const GRAVITY: Vec2 = Vec2::new(0.0, -9.81);
const PHYSICS_FRAMERATE: f64 = 60.0;

const BACKGROUND_COLOR: Color = Color::Rgba {
    red: 0.18,
    green: 0.21,
    blue: 0.20,
    alpha: 1.0,
};

pub fn main() {
    eprintln!("Starting pollywog...");

    App::new()
        .init_state::<GameState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / PHYSICS_FRAMERATE))
        .add_plugins((
            setup_default_plugins(),
            perf_ui_plugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER)
                .in_fixed_schedule(),
            RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<components::character::Action>::default(),
            camera::camera_plugin,
            plugins::screens_plugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(LogPlugin {
            filter: format!(
                "info,wgpu_core=warn,wgpu_hal=warn,pollywog={}",
                std::env::var("POLLYWOG_LOG_LEVEL").unwrap_or("info".to_string())
            ),
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
        .set(ImagePlugin::default_nearest())
}

fn perf_ui_plugin(app: &mut App) {
    app.add_plugins((
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
        PerfUiPlugin,
    ))
    .add_systems(Startup, |mut commands: Commands| {
        commands.spawn(PerfUiCompleteBundle::default());
    });
}

// FIXME: oh my god this is so fragile
pub mod z_index {
    // FIXME: rename all hills/islands to match z order
    type ZIndex = f32;
    pub const BG_BASE: ZIndex = 0.0;
    pub const BG_MAX: ZIndex = 8.0;
    pub const SWAMP_KELP_1: ZIndex = BG_BASE + 7.0;
    pub const SWAMP_KELP_0: ZIndex = BG_BASE + 6.0;
    pub const SWAMP_ISLAND_0: ZIndex = BG_BASE + 5.0;
    pub const SWAMP_ISLAND_1: ZIndex = BG_BASE + 4.0;
    pub const SWAMP_ISLAND_2: ZIndex = BG_BASE + 3.0;
    pub const SWAMP_HILLS_0: ZIndex = BG_BASE + 2.0;
    pub const SWAMP_HILLS_1: ZIndex = BG_BASE + 1.0;
    pub const SWAMP_HILLS_2: ZIndex = BG_BASE + 0.0;
    pub const SWAMP_POND: ZIndex = BG_BASE + 8.0;

    pub const LEVEL_BASE: ZIndex = BG_MAX + 1.0;
}
