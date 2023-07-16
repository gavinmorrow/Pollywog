use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{Action, Player};

#[derive(Default)]
pub struct GrapplePlugin {}

impl Plugin for GrapplePlugin {
    fn build(&self, app: &mut App) {
        debug!("Building GrapplePlugin");

        app.add_state::<GrappleState>()
            .add_systems(OnEnter(GrappleState::Grappling), start_grapple)
            .add_systems(
                Update,
                (
                    aim.run_if(state_exists_and_equals(GrappleState::Aiming)),
                    watch_input,
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

fn aim(
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut _player) = player_query.get_single_mut() else { return; };
    let window = window_query.single();
    let Some(mouse_pos) = window.cursor_position() else {
		warn!("Tried to aim grapple when mouse was not in window");
		return;
	};

    debug!("Aiming grapple");
    trace!("Aiming grapple (mouse pos: {:?})", mouse_pos);
}

pub fn watch_input(
    action_state_query: Query<&ActionState<Action>, With<Player>>,
    grapple_state: Res<State<GrappleState>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let action_state = action_state_query.single();
    let grapple_state = grapple_state.get();

    match grapple_state {
        GrappleState::Idle => {
            let just_pressed = action_state.get_just_pressed();
            let just_released = action_state.get_just_released();

            // Only start aiming if grapple was just pressed and not released
            if just_pressed.contains(&Action::Grapple) && !just_released.contains(&Action::Grapple)
            {
                debug!("Starting grapple aiming (idle -> aiming)");
                next_grapple_state.set(grapple_state.next());
            }
        }
        GrappleState::Aiming => {
            let just_released = action_state.get_just_released();

            if just_released.contains(&Action::Grapple) {
                debug!("Starting grapple (aiming -> grappling)");
                next_grapple_state.set(grapple_state.next());
            }
        }
        GrappleState::Grappling => {
            // TODO: act as breaking out of grapple
            // Maybe start new grapple? ask mateo
            debug!("Reeling in grapple");
            next_grapple_state.set(GrappleState::Idle);
        }
    }
}

fn start_grapple(
    spatial_query: SpatialQuery,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<(Entity, &Collider, &Transform), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    entities_query: Query<&Transform>,
    mut commands: Commands,
) {
    // Resolve queries
    let window = window_query.single();
    let Ok((player, player_collider, player_transform)) = player_query.get_single() else {
		error!("Could not get player collider or transform");
		return;
	};
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
		error!("Could not get camera for starting grapple");
		return;
	};

    debug!("Starting grapple shapecast");

    // Cast shape
    let Ok(direction) = resolve_mouse_pos(
        window,
        camera,
        camera_transform,
        player_transform.translation.truncate(),
    ) else {
		error!("Could not resolve mouse position for starting grapple");
		return;
	};
    let origin = player_transform.translation.truncate();
    let distance_to_window_edge = get_distance_to_window_edge(player_transform, window, direction);
    let query_filter = SpatialQueryFilter::default().without_entities([player]);

    let Some(first_hit) = spatial_query.cast_shape(
		player_collider,
		origin,
		default(),
		direction,
		distance_to_window_edge,
		true,
		query_filter,
	) else {
		warn!("Grapple shapecast hit nothing");
		return;
	};

    // Find transform for entity
    let Ok(entity_transform) = entities_query.get(first_hit.entity) else {
		error!("Grapple shapecast hit entity with no transform");
		return;
	};

    // Resolve point to global space
    let point = resolve_local_point(&entity_transform.translation.truncate(), &first_hit.point2);

    debug!("Grapple shapecast hit: {:?}", point);

    // for debugging, add a point at the hit locations
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        transform: Transform::from_translation(point.extend(1.0)),
        ..Default::default()
    });
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

fn resolve_local_point(translate: &Vec2, local_point: &Vec2) -> Vec2 {
    let result = *translate - *local_point;

    trace!(
        "Resolving local point (translate: {:?}, local_point: {:?}) to {:?}",
        translate,
        local_point,
        result
    );

    result
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

    Ok(direction)
}
