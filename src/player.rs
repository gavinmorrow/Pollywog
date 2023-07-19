use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub mod grapple;

const SIZE: f32 = 64.0;
const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

const TEXTURE_PATH: &str = "player.png";

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    collider: Collider,
    player: Player,
    angular_damping: AngularDamping,
    linear_damping: LinearDamping,
    input_manager: InputManagerBundle<Action>,
    external_force: ExternalForce,
    gravity_scale: GravityScale,
}

impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>, window: &Window) -> Self {
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
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(SIZE / 2.0),
            player: Player,
            angular_damping: AngularDamping(3.0),
            linear_damping: LinearDamping(1.0),
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

pub fn spawn(
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
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
) {
    let action_state = action_state_query.single();
    let Ok(mut player) = player_query.get_single_mut() else { return; };
    let actions = action_state.get_pressed();

    if actions.len() > 0 {
        debug!("Moving player.");
    }

    for action in actions {
        trace!("Action: {:#?}", action);
        match action {
            Action::Left => player.x = -300.0,
            Action::Right => player.x = 300.0,
            Action::Jump => player.y = 300.0,
            Action::Grapple => { /* Do nothing, this is handled elsewhere. */ }
        }
    }
}

/// Add a force to the player in the given direction (to be used for grappling).
fn add_grapple_force(
    mut player_query: Query<(&mut ExternalForce, &mut GravityScale), With<Player>>,
    direction: Vec2,
) {
    let (mut external_force, mut gravity) = player_query.single_mut();

    // Add force to player
    let force = direction * grapple::FORCE_MULT;
    trace!("Setting external force on player to: {:?}", force);
    external_force.set_force(force);

    // Remove player gravity
    gravity.0 = 0.0;

    debug!("Added grapple force to player.");
}

/// Remove the force from the player (to be used for stopping grappling).
fn remove_grapple_force(
    mut player_query: Query<(&mut ExternalForce, &mut GravityScale), With<Player>>,
) {
    let (mut external_force, mut gravity) = player_query.single_mut();

    // Remove player external force
    external_force.set_force(Vec2::ZERO);

    // Add player gravity
    gravity.0 = 1.0;

    debug!("Removed grapple force from player.");
}
