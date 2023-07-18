use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{Action, Player};

pub const FORCE_MULT: f32 = 5_000_000.0;

#[derive(Default)]
pub struct GrapplePlugin {}

impl Plugin for GrapplePlugin {
    fn build(&self, app: &mut App) {
        debug!("Building GrapplePlugin");

        app.add_state::<GrappleState>()
            .add_systems(OnExit(GrappleState::Grappling), end_grapple)
            .add_systems(
                Update,
                (
                    idle.run_if(state_exists_and_equals(GrappleState::Idle)),
                    aim.run_if(state_exists_and_equals(GrappleState::Aiming)),
                    aim_guideline.run_if(state_exists_and_equals(GrappleState::Aiming)),
                    grapple.run_if(state_exists_and_equals(GrappleState::Grappling)),
                    manage_grapple.run_if(state_exists_and_equals(GrappleState::Grappling)),
                    should_grapple_end.run_if(state_exists_and_equals(GrappleState::Grappling)),
                    end_grapple_on_other_input
                        .run_if(state_exists_and_equals(GrappleState::Grappling)),
                ),
            );
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GrappleState {
    Idle,
    Aiming,
    Grappling,
}

impl GrappleState {
    pub fn next(&self) -> Self {
        match self {
            Self::Idle => Self::Aiming,
            Self::Aiming => Self::Grappling,
            Self::Grappling => Self::Idle,
        }
    }
}

impl Default for GrappleState {
    fn default() -> Self {
        Self::Idle
    }
}

fn idle(
    action_state_query: Query<&ActionState<Action>, With<Player>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let action_state = action_state_query.single();

    let just_pressed = action_state.get_just_pressed();
    let just_released = action_state.get_just_released();

    // Only start aiming if grapple was just pressed and not released
    if just_pressed.contains(&Action::Grapple) && !just_released.contains(&Action::Grapple) {
        debug!("Starting grapple aiming (idle -> aiming)");
        next_grapple_state.set(GrappleState::Idle.next());
    }
}

fn aim(
    action_state_query: Query<&ActionState<Action>, With<Player>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let action_state = action_state_query.single();
    let just_released = action_state.get_just_released();

    // If the key was just released, stop aiming and start grappling
    if just_released.contains(&Action::Grapple) {
        debug!("Starting grapple (aiming -> grappling)");
        next_grapple_state.set(GrappleState::Aiming.next());
    }
}

fn aim_guideline(
    spatial_query: SpatialQuery,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
) {
    let Ok((point, target)) = cast_grapple_ray(spatial_query, window_query, player_query, camera_query) else {
        warn!("Could not cast grapple ray");
        return;
    };

    // Add grapple marker
    add_grapple_marker(&mut commands, &point);

    // Add point to target pos resource
    commands.insert_resource(TargetPos(point, target));
}

enum RaycastError {
    NoPlayer,
    NoCamera,
    CouldNotResolveMousePos,
    RayHitNothing,
}

/// Casts a ray from the player to the mouse position.
///
/// # Returns
///
/// A tuple, containing:
///
/// 0. the point of impact
/// 1. the entity that was impacted
///
/// An error is returned if there was an error casting the ray, or if the ray
/// hit nothing.
fn cast_grapple_ray(
    spatial_query: SpatialQuery,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) -> Result<(Vec2, Entity), RaycastError> {
    // Resolve queries
    let window = window_query.single();
    let Ok((player, player_transform)) = player_query.get_single() else {
		error!("Could not get player entity or transform");
		return Err(RaycastError::NoPlayer);
	};
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
		error!("Could not get camera for grapple raycast");
		return Err(RaycastError::NoCamera);
	};

    debug!("Starting grapple raycast");

    // Get ray input
    let Ok(direction) = resolve_mouse_pos(
        window,
        camera,
        camera_transform,
        player_transform.translation.truncate(),
    ) else {
		error!("Could not resolve mouse position for starting grapple");
		return Err(RaycastError::CouldNotResolveMousePos);
	};
    let origin = player_transform.translation.truncate();
    let distance_to_window_edge = get_distance_to_window_edge(player_transform, window, direction);
    let query_filter = SpatialQueryFilter::default().without_entities([player]);

    trace!(
        "Origin: {}, direction: {}, distance_to_window_edge: {}",
        origin,
        direction,
        distance_to_window_edge
    );

    // Cast ray
    let Some(first_hit) = spatial_query.cast_ray(
        origin,
        direction,
        distance_to_window_edge,
        true,
        query_filter,
    ) else {
		warn!("Raycast hit nothing");
		return Err(RaycastError::RayHitNothing);
	};
    let point = origin + direction * first_hit.time_of_impact;
    let entity = first_hit.entity;

    debug!("Raycast hit entity {:?} at {:?}", entity, point);

    Ok((point, entity))
}

#[derive(Resource)]
struct TargetPos(Vec2, Entity);

fn grapple(
    action_state_query: Query<&ActionState<Action>, With<Player>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let action_state = action_state_query.single();
    let just_pressed = action_state.get_just_pressed();

    // If the grapple key was just pressed, stop grappling and start aiming
    if just_pressed.contains(&Action::Grapple) {
        debug!("Stopping grapple (grappling -> aiming)");
        next_grapple_state.set(GrappleState::Aiming);
    }
}

fn manage_grapple(
    player_query: Query<&GlobalTransform, With<Player>>,
    target_pos: Option<ResMut<TargetPos>>,
    mut_player_query: Query<(&mut ExternalForce, &mut GravityScale), With<Player>>,
) {
    // Resolve queries
    let Ok(player_transform) = player_query.get_single() else {
		error!("Could not get player transform");
		return;
	};
    let Some(target_pos) = target_pos else {
		error!("Could not get target position");
		return;
	};

    let player = player_transform.translation().truncate();
    let target = target_pos.0;

    // Recalculate the direction to the target
    let direction = target - player;
    let direction = direction.normalize();

    debug!("Recalculated grapple direction to {:?}", direction);

    // Set the force on the player
    super::add_grapple_force(mut_player_query, direction);
}

fn should_grapple_end(
    mut collisions: EventReader<Collision>,
    player: Query<Entity, With<Player>>,
    target_pos: Option<Res<TargetPos>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let player = player.single();

    let Some(target_pos) = target_pos else {
		warn!("No target pos resource");
		return;
	};
    let target = target_pos.1;

    // Check if the player is touching the target
    for collision in collisions.iter() {
        if (collision.0.entity1 == player && collision.0.entity2 == target)
            || (collision.0.entity2 == player && collision.0.entity1 == target)
        {
            debug!("Player is touching target, stopping grapple");
            next_grapple_state.set(GrappleState::Grappling.next());

            // No more cleanup is needed because it will be done in the OnExit
            return;
        }
    }

    trace!("Player is not touching target ({:?})", target_pos.0);
}

fn end_grapple_on_other_input(
    action_state_query: Query<&ActionState<Action>, With<Player>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let action_state = action_state_query.single();
    for action in action_state.get_pressed() {
        if action != Action::Grapple {
            // End grapple
            next_grapple_state.set(GrappleState::Grappling.next());
        }
    }
}

fn add_grapple_marker(commands: &mut Commands, point: &Vec2) -> Entity {
    // add a point at the hit location
    debug!("Adding grapple marker at {:?}", point);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_translation(point.extend(1.0)),
            ..Default::default()
        })
        .id()
}

fn end_grapple(
    mut_player_query: Query<(&mut ExternalForce, &mut GravityScale), With<Player>>,
    mut commands: Commands,
) {
    debug!("Ending grapple");

    // Remove target pos resource
    commands.remove_resource::<TargetPos>();

    // Remove player external force
    super::remove_grapple_force(mut_player_query);
}

fn get_distance_to_window_edge(player: &Transform, window: &Window, direction: Vec2) -> f32 {
    let window_size = Vec2::new(window.width(), window.height());
    let player_pos = player.translation.truncate();

    let distance_to_edge = window_size - player_pos;
    let distance_to_edge = distance_to_edge / direction;
    let distance_to_edge = (distance_to_edge.x.powf(2.0) + distance_to_edge.y.powf(2.0)).sqrt();
    let distance_to_edge = distance_to_edge / 2.0;

    distance_to_edge
}

enum ResolveMousePosError {
    NoMousePos,
    NoMouseCoords,
}

fn resolve_mouse_pos(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    player_translation: Vec2,
) -> Result<Vec2, ResolveMousePosError> {
    // Get mouse_pos relative to top left of screen
    let Some(mouse_pos) = window.cursor_position() else {
		warn!("Tried to start grapple when mouse was not in window");
		return Err(ResolveMousePosError::NoMousePos);
	};

    // Make mouse_pos relative to world (not top left of screen)
    let Some(mouse_pos) = camera.viewport_to_world_2d(camera_transform, mouse_pos) else {
		error!("Could not get mouse position in world space");
		return Err(ResolveMousePosError::NoMouseCoords);
	};

    // Make mouse_pos relative to player (not world)
    let direction = mouse_pos - player_translation;
    let direction = direction.normalize();

    Ok(direction)
}
