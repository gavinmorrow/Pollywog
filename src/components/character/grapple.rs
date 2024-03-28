use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    bundles::player::Player,
    components::character::{add_grapple_force, Action, Character},
    state::GameState,
};

const GUIDELINE_DISTANCE: f32 = 50.0;
const GUIDELINE_SIZE: f32 = 10.0;

pub fn grapple_plugin(app: &mut App) {
    debug!("Building GrapplePlugin");

    app.init_state::<GrappleState>()
        .insert_resource(Guideline::default())
        .add_systems(OnExit(GrappleState::Grappling), end_grapple)
        .add_systems(OnExit(GrappleState::Aiming), remove_guideline_system)
        .add_systems(
            Update,
            (
                idle.run_if(in_state(GrappleState::Idle)),
                aim.run_if(in_state(GrappleState::Aiming)),
                aim_marker.run_if(in_state(GrappleState::Aiming)),
                aim_guideline.run_if(in_state(GrappleState::Aiming)),
                grapple.run_if(in_state(GrappleState::Grappling)),
                manage_grapple.run_if(in_state(GrappleState::Grappling)),
                should_grapple_end.run_if(in_state(GrappleState::Grappling)),
                end_grapple_on_other_input.run_if(in_state(GrappleState::Grappling)),
            )
                .run_if(in_state(GameState::InGame)),
        );
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum GrappleState {
    #[default]
    Idle,
    Aiming,
    Grappling,
}

impl GrappleState {
    fn next(&self) -> Self {
        match self {
            Self::Idle => Self::Aiming,
            Self::Aiming => Self::Grappling,
            Self::Grappling => Self::Idle,
        }
    }
}

#[derive(Resource)]
struct TargetPos(
    /// The position of the target.
    Vec2,
    /// The entity that the target is attached to.
    Entity,
    /// The entity of the target.
    Entity,
);

#[derive(Resource, Default)]
struct Guideline(Vec<Entity>);

pub fn cleanup(mut next_grapple_state: ResMut<NextState<GrappleState>>) {
    next_grapple_state.set(GrappleState::default());
}

fn idle(
    action_state_query: Query<&ActionState<Action>, With<Character>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let action_state = action_state_query.single();

    let just_pressed = action_state.get_just_pressed();
    let just_released = action_state.get_just_released();

    // Only start aiming if grapple was just pressed and not released
    if just_pressed.contains(&Action::Grapple) && !just_released.contains(&Action::Grapple) {
        info!("Starting grapple aiming (idle -> aiming)");
        next_grapple_state.set(GrappleState::Idle.next());
    }
}

fn aim(
    action_state_query: Query<&ActionState<Action>, With<Character>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let action_state = action_state_query.single();
    let just_released = action_state.get_just_released();

    // If the key was just released, stop aiming and start grappling
    if just_released.contains(&Action::Grapple) {
        info!("Starting grapple (aiming -> grappling)");
        next_grapple_state.set(GrappleState::Aiming.next());
    }
}

fn aim_marker(
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    char_query: Query<(Entity, &Transform), With<Character>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    target_pos: Option<Res<TargetPos>>,
    mut commands: Commands,
) {
    // Clear old marker
    if let Some(target_pos) = target_pos {
        remove_target_pos(&mut commands, target_pos.2);
    }

    let Ok((point, target)) =
        cast_grapple_ray(rapier_context, window_query, char_query, camera_query)
    else {
        trace!("No result for grapple raycast");
        return;
    };

    // Add grapple marker
    let marker = add_grapple_marker(&mut commands, &point);

    // Add point to target pos resource
    let target_pos = TargetPos(point, target, marker);
    commands.insert_resource(target_pos);
}

fn aim_guideline(
    target_pos: Option<Res<TargetPos>>,
    char_query: Query<&GlobalTransform, With<Character>>,
    mut guideline: ResMut<Guideline>,
    mut commands: Commands,
) {
    // Clear old guidelines
    remove_guideline(&mut guideline, &mut commands);

    let Some(target_pos) = target_pos else {
        trace!("No target pos for grapple guidelines");
        return;
    };
    let char = char_query.single();

    // Get direction from character to target
    let char_pos = char.translation().truncate();
    let target_pos = target_pos.0;
    let direction = target_pos - char_pos;
    let distance = direction.normalize() * GUIDELINE_DISTANCE;

    // Add guidelines
    let mut current_pos = char_pos;
    while current_pos.distance(target_pos) >= GUIDELINE_DISTANCE {
        let sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(GUIDELINE_SIZE, GUIDELINE_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: current_pos.extend(2.0),
                ..default()
            },
            ..default()
        };

        let entity = commands.spawn(sprite).id();

        guideline.0.push(entity);
        current_pos += distance;
    }
}

/// Cleanly removes the `TargetPos` resource.
///
/// This despawns the marker entity and removes the resource.
fn remove_target_pos(commands: &mut Commands, marker: Entity) {
    trace!("Removing target pos for marker {:?}", marker);
    commands.entity(marker).despawn_recursive();
    commands.remove_resource::<TargetPos>();
}

enum RaycastError {
    NoCharacter,
    NoCamera,
    CouldNotResolveMousePos,
    RayHitNothing,
}

/// Casts a ray from the character to the mouse position.
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
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    char_query: Query<(Entity, &Transform), With<Character>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) -> Result<(Vec2, Entity), RaycastError> {
    // Resolve queries
    let window = window_query.single();
    let Ok((char, char_transform)) = char_query.get_single() else {
        error!("Could not get character entity or transform");
        return Err(RaycastError::NoCharacter);
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        error!("Could not get camera for grapple raycast");
        return Err(RaycastError::NoCamera);
    };

    // Get ray input
    let Ok(direction) = resolve_mouse_pos(
        window,
        camera,
        camera_transform,
        char_transform.translation.truncate(),
    ) else {
        trace!("Could not resolve mouse position for starting grapple");
        return Err(RaycastError::CouldNotResolveMousePos);
    };
    let origin = char_transform.translation.truncate();
    let distance_to_window_edge = get_distance_to_window_edge(char_transform, window, direction);
    // FIXME: exclude character from raycast
    let query_filter = QueryFilter {
        exclude_collider: Some(char),
        ..default()
    };

    trace!(
        "Origin: {}, direction: {}, distance_to_window_edge: {}",
        origin,
        direction,
        distance_to_window_edge
    );

    // Cast ray
    let Some((entity, toi)) = rapier_context.cast_ray(
        origin,
        direction,
        distance_to_window_edge,
        true,
        query_filter,
    ) else {
        trace!("Raycast hit nothing");
        return Err(RaycastError::RayHitNothing);
    };
    let point = origin + direction * toi;

    trace!("Raycast hit entity {:?} at {:?}", entity, point);

    Ok((point, entity))
}

fn grapple(
    action_state_query: Query<&ActionState<Action>, With<Character>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let action_state = action_state_query.single();
    let just_pressed = action_state.get_just_pressed();

    // If the grapple key was just pressed, stop grappling and start aiming
    if just_pressed.contains(&Action::Grapple) {
        info!("Stopping grapple (grappling -> aiming)");
        next_grapple_state.set(GrappleState::Aiming);
    }
}

fn manage_grapple(
    char_query: Query<&GlobalTransform, With<Character>>,
    target_pos: Option<ResMut<TargetPos>>,
    char_controller_query: Query<&mut KinematicCharacterController, With<Character>>,
    mut sprite_query: Query<&mut Sprite, With<Character>>,
) {
    // Resolve queries
    let Ok(char_transform) = char_query.get_single() else {
        error!("Could not get character transform");
        return;
    };
    let Some(target_pos) = target_pos else {
        trace!("Could not get target pos resource");
        return;
    };

    let char = char_transform.translation().truncate();
    let target = target_pos.0;

    // Recalculate the direction to the target
    let direction = target - char;
    let direction = direction.normalize();

    trace!("Recalculated grapple direction to {:?}", direction);

    sprite_query.single_mut().flip_x = direction.x < 0.0;

    // Set the force on the character
    add_grapple_force(char_controller_query, direction);
}

fn should_grapple_end(
    player: Query<&KinematicCharacterControllerOutput, With<Player>>,
    target_pos: Option<Res<TargetPos>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let player = player.single();
    let Some(target_pos) = target_pos else {
        trace!("No target pos resource");

        // End grapple
        info!("Ending grapple because of missing target pos (grappling -> idle)");
        next_grapple_state.set(GrappleState::Grappling.next());

        return;
    };
    let target = target_pos.1;

    // Check if the character is touching the target
    for collision in &player.collisions {
        if collision.entity == target {
            // End grapple
            info!("Ending grapple because character is touching target (grappling -> idle)");
            next_grapple_state.set(GrappleState::Grappling.next());
            return;
        }
    }

    trace!("Character is not touching target ({:?})", target_pos.0);
}

fn end_grapple_on_other_input(
    action_state_query: Query<&ActionState<Action>, With<Character>>,
    mut next_grapple_state: ResMut<NextState<GrappleState>>,
) {
    let action_state = action_state_query.single();
    for action in action_state.get_pressed() {
        if action != Action::Grapple {
            // End grapple
            info!("Ending grapple because of input (grappling -> idle)");
            next_grapple_state.set(GrappleState::Grappling.next());
        }
    }
}

fn add_grapple_marker(commands: &mut Commands, point: &Vec2) -> Entity {
    // add a point at the hit location
    trace!("Adding grapple marker at {:?}", point);
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

fn end_grapple(target_pos: Option<Res<TargetPos>>, mut commands: Commands) {
    debug!("Ending grapple");

    // Remove target pos resource if it exists
    if let Some(target_pos) = target_pos {
        remove_target_pos(&mut commands, target_pos.2);
    }
}

fn remove_guideline(guideline: &mut ResMut<Guideline>, commands: &mut Commands) {
    trace!("Removing guideline");

    // Clear old guidelines
    for guideline in &guideline.0 {
        commands.entity(*guideline).despawn();
    }

    guideline.0.clear();
}

fn remove_guideline_system(mut guideline: ResMut<Guideline>, mut commands: Commands) {
    remove_guideline(&mut guideline, &mut commands);
}

fn get_distance_to_window_edge(char: &Transform, window: &Window, direction: Vec2) -> f32 {
    let window_size = Vec2::new(window.width(), window.height());
    let char_pos = char.translation.truncate();

    let distance_to_edge = window_size - char_pos;
    let distance_to_edge = distance_to_edge / direction;
    let distance_to_edge = (distance_to_edge.x.powf(2.0) + distance_to_edge.y.powf(2.0)).sqrt();

    distance_to_edge / 2.0
}

enum ResolveMousePosError {
    NoMousePos,
    NoMouseCoords,
}

fn resolve_mouse_pos(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    char_translation: Vec2,
) -> Result<Vec2, ResolveMousePosError> {
    // Get mouse_pos relative to top left of screen
    let Some(mouse_pos) = window.cursor_position() else {
        trace!("Tried to start grapple when mouse was not in window");
        return Err(ResolveMousePosError::NoMousePos);
    };

    // Make mouse_pos relative to world (not top left of screen)
    let Some(mouse_pos) = camera.viewport_to_world_2d(camera_transform, mouse_pos) else {
        error!("Could not get mouse position in world space");
        return Err(ResolveMousePosError::NoMouseCoords);
    };

    // Make mouse_pos relative to character (not world)
    let direction = mouse_pos - char_translation;
    let direction = direction.normalize();

    Ok(direction)
}
