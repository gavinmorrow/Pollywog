use crate::components::character::{Action, Character};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{health::Health, jump::JumpComponent},
    GRAVITY,
};

const SIZE: f32 = 64.0;
const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

const JUMP_MAGNITUDE: Vec2 = Vec2::new(0.0, 10.0);
const MOVEMENT_SPEED: f32 = 3.0;

const TEXTURE_PATH: &str = "player.png";

const INITIAL_HEALTH: f32 = 100.0;

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    character_controller: KinematicCharacterController,
    collider: Collider,
    player: Player,
    damping: Damping,
    input_manager: InputManagerBundle<Action>,
    external_force: ExternalForce,
    gravity_scale: GravityScale,
    jump_component: JumpComponent,
    char: Character,
    health: Health,
}

impl PlayerBundle {
    fn new(asset_server: Res<AssetServer>, window: &Window) -> Self {
        debug!("Creating player bundle");

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, window.height(), 0.0),
                    ..default()
                },
                texture: asset_server.load(TEXTURE_PATH),
                sprite: Sprite {
                    custom_size: Some(SIZE_VEC2),
                    ..default()
                },
                ..default()
            },
            // FIXME: maybe not default? just trying to get this to work for now
            character_controller: KinematicCharacterController {
                translation: Some(GRAVITY),
                ..default()
            },
            collider: Collider::ball(SIZE / 2.0),
            player: Player,
            damping: Damping {
                angular_damping: 3.0,
                linear_damping: 0.0,
            },
            input_manager: InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: get_input_map(),
            },
            external_force: ExternalForce::default(),
            gravity_scale: GravityScale(1.0),
            jump_component: JumpComponent::new(JUMP_MAGNITUDE, false),
            char: Character {
                movement_speed: MOVEMENT_SPEED,
            },
            health: Health::full(INITIAL_HEALTH),
        }
    }
}

fn get_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();

    #[rustfmt::skip]
        input_map
            .insert(KeyCode::Left,         Action::Left)
            .insert(QwertyScanCode::A,     Action::Left)
            .insert(KeyCode::Right,        Action::Right)
            .insert(QwertyScanCode::D,     Action::Right)
            .insert(KeyCode::Up,           Action::Jump)
            .insert(QwertyScanCode::W,     Action::Jump)
            .insert(KeyCode::Space,        Action::Jump)
            .insert(QwertyScanCode::E,     Action::Grapple)
            .insert(QwertyScanCode::Slash, Action::Grapple);

    input_map
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    debug!("Spawning player");
    commands.spawn(PlayerBundle::new(asset_server, window));
}
