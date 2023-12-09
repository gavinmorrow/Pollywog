use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

mod grapple;

const SIZE: f32 = 64.0;
const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

const TEXTURE_PATH: &str = "player.png";

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(grapple::GrapplePlugin::default())
            .insert_resource(CanJump(true))
            .add_systems(Startup, spawn)
            .add_systems(Update, (r#move, can_jump.before(r#move)));
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    // FIXME: might need a rigid body
    character_controller: KinematicCharacterController,
    collider: Collider,
    player: Player,
    damping: Damping,
    input_manager: InputManagerBundle<Action>,
    external_force: ExternalForce,
    gravity_scale: GravityScale,
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
            character_controller: KinematicCharacterController::default(),
            collider: Collider::ball(SIZE / 2.0),
            player: Player,
            damping: Damping {
                angular_damping: 3.0,
                linear_damping: 1.0,
            },
            input_manager: InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map,
            },
            external_force: ExternalForce::default(),
            gravity_scale: GravityScale(1.0),
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

#[derive(Resource, Default)]
pub struct CanJump(pub bool);

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    debug!("Spawning player");
    commands.spawn(PlayerBundle::new(asset_server, window));
}

// FIXME: make a better movement system, this is just a placeholder
pub fn r#move(
    action_state_query: Query<&ActionState<Action>, With<Player>>,
    can_jump: Res<CanJump>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
) {
    let action_state = action_state_query.single();
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };
    let actions = action_state.get_pressed();

    if !actions.is_empty() {
        debug!("Moving player.");
    }

    let Some(translation) = &mut player.translation else {
        return;
    };

    for action in actions {
        trace!("Action: {:#?}", action);
        match action {
            Action::Left => translation.x = -300.0,
            Action::Right => translation.x = 300.0,
            Action::Jump => {
                if can_jump.0 {
                    translation.y = 300.0
                }
            }
            Action::Grapple => { /* Do nothing, this is handled elsewhere. */ }
        }
    }
}

fn can_jump(
    mut collisions: EventReader<CollisionEvent>,
    mut can_jump: ResMut<CanJump>,
    query: Query<(
        Entity,
        Option<&crate::level::block::JumpCollisionBox>,
        Option<&Player>,
    )>,
) {
    let player = query
        .iter()
        .find(|(_, _, player)| player.is_some())
        .unwrap()
        .0;
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(a, b, flags) => {
                let a = *a;
                let b = *b;

                if player == a || player == b {
                    let other = if player == a { b } else { a };

                    if query.get(other).is_ok() {
                        can_jump.0 = true;
                        trace!("Player can jump.");
                        return;
                    }
                }
            }
            // Do nothing, we don't care about stopped collisions
            // FIXME: maybe we should care about stopped collisions?
            // Possibly could move `can_jump.0 = false` up here?
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
    can_jump.0 = false;
    trace!("Player can't jump.");
}

/// Add a force to the player in the given direction (to be used for grappling).
fn add_grapple_force(
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
    direction: Vec2,
) {
    let character_controller = &mut player_query.single_mut();
    let Some(translation) = &mut character_controller.translation else {
        debug!("No translation found for player, can't add grapple force.");
        return;
    };

    // Add force to player
    // FIXME: gravity might still be applied here, idk
    let force = direction;
    *translation = force;
    trace!("Setting external force on player to: {:?}", force);

    debug!("Added grapple force to player.");
}

/// Remove the force from the player (to be used for stopping grappling).
fn remove_grapple_force(mut player_query: Query<&mut KinematicCharacterController, With<Player>>) {
    let character_controller = &mut player_query.single_mut();
    let Some(translation) = &mut character_controller.translation else {
        debug!("No translation found for player, can't add grapple force.");
        return;
    };

    // Remove player external force
    // FIXME: gravity needs to be applied here
    let force = Vec2::ZERO;
    *translation = force;
    trace!("Setting external force on player to: {:?}", force);

    debug!("Removed grapple force from player.");
}
