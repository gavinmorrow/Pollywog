use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{jump_component::JumpComponent, GRAVITY};

mod grapple;

const SIZE: f32 = 64.0;
const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

const JUMP_MAGNITUDE: Vec2 = Vec2::new(0.0, 10.0);
const MOVEMENT_SPEED: f32 = 3.0;

const TEXTURE_PATH: &str = "player.png";

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(grapple::GrapplePlugin::default())
            .add_systems(Startup, spawn)
            // FIXME: maybe move the jump system somewhere else
            .add_systems(
                Update,
                (
                    r#move,
                    // Must go after so that the player gets moved immediately after
                    // the jump starts
                    crate::jump_component::jump.after(r#move),
                    // Must go before so that the player is off the ground when we check
                    stop_jump.before(r#move),
                ),
            );
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
}

impl PlayerBundle {
    fn new(asset_server: Res<AssetServer>, window: &Window) -> Self {
        debug!("Creating player bundle");

        let mut input_map = InputMap::default();
        input_map
            .insert(KeyCode::Left, Action::Left)
            .insert(QwertyScanCode::A, Action::Left)
            .insert(KeyCode::Right, Action::Right)
            .insert(QwertyScanCode::D, Action::Right)
            .insert(KeyCode::W, Action::Jump)
            .insert(QwertyScanCode::Up, Action::Jump)
            .insert(KeyCode::Space, Action::Jump)
            .insert(QwertyScanCode::E, Action::Grapple)
            .insert(QwertyScanCode::Slash, Action::Grapple);

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
                input_map,
            },
            external_force: ExternalForce::default(),
            gravity_scale: GravityScale(1.0),
            jump_component: JumpComponent::new(JUMP_MAGNITUDE, false),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Left,
    Right,
    Jump,
    Grapple,
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

pub fn r#move(
    action_state_query: Query<&ActionState<Action>, With<Player>>,
    mut player_query: Query<
        (
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
        ),
        With<Player>,
    >,
    mut jump_component_query: Query<&mut JumpComponent, With<Player>>,
) {
    let action_state = action_state_query.single();
    let Ok((mut player, kinematic_character_controller_output)) = player_query.get_single_mut()
    else {
        return;
    };
    let actions = action_state.get_pressed();

    if !actions.is_empty() {
        trace!("Moving player.");
    }

    player.translation = Some(player.translation.unwrap_or(GRAVITY));
    let translation = &mut player.translation.expect("Just set to a Some value above.");

    for action in actions {
        trace!("Action: {:#?}", action);
        match action {
            Action::Left => translation.x = -MOVEMENT_SPEED,
            Action::Right => translation.x = MOVEMENT_SPEED,
            Action::Jump => {
                if kinematic_character_controller_output.grounded {
                    trace!("Player is grounded, starting jump.");
                    let mut jump_component = jump_component_query.single_mut();
                    jump_component.start_jump();
                } else {
                    trace!("Player is not grounded, can't jump.");
                }
            }
            Action::Grapple => { /* Do nothing, this is handled elsewhere. */ }
        }
    }

    player.translation = Some(*translation);
}

pub fn stop_jump(
    mut player_query: Query<
        (&mut JumpComponent, &KinematicCharacterControllerOutput),
        With<Player>,
    >,
) {
    let Ok((mut jump_component, output)) = player_query.get_single_mut() else {
        return;
    };

    if output.grounded && jump_component.is_jumping() {
        debug!("Player is grounded, stopping jump.");
        jump_component.stop_jump();
    }
}

/// Add a force to the player in the given direction (to be used for grappling).
fn add_grapple_force(
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
    direction: Vec2,
) {
    let character_controller = &mut player_query.single_mut();

    // Completely replace player translation with grapple force
    let force = direction * 10.0;
    character_controller.translation = Some(force);
    trace!("Setting grapple force on player to: {:?}", force);
}

/// Remove the force from the player (to be used for stopping grappling).
fn remove_grapple_force(mut player_query: Query<&mut KinematicCharacterController, With<Player>>) {
    let character_controller = &mut player_query.single_mut();
    let Some(translation) = &mut character_controller.translation else {
        warn!("No translation found for player, can't remove grapple force.");
        return;
    };

    // Remove player external force
    let force = GRAVITY;
    *translation = force;
    trace!("Setting external force on player to: {:?}", force);

    debug!("Removed grapple force from player.");
}
