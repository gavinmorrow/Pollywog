use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use jump::JumpComponent;
use leafwing_input_manager::prelude::*;

use crate::{bundles::player, state::GameState, GRAVITY};

use super::animated_sprite::CurrentlyAnimating;

pub mod grapple;
pub mod jump;

pub fn character_plugin(app: &mut App) {
    app.add_plugins(crate::components::character::grapple::grapple_plugin)
        // FIXME: maybe move the jump system somewhere else
        .add_systems(
            Update,
            (stop_jump, r#move, jump::jump)
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
}

pub fn cleanup(next_grapple_state: ResMut<NextState<grapple::GrappleState>>) {
    grapple::cleanup(next_grapple_state);
}

#[derive(Component, Default)]
pub struct Character {
    pub movement_speed: f32,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Left,
    Right,
    Jump,
    Grapple,
}

pub fn r#move(
    action_state_query: Query<&ActionState<Action>, With<Character>>,
    mut player_query: Query<(
        Entity,
        &mut KinematicCharacterController,
        &mut Sprite,
        &Character,
        &mut CurrentlyAnimating,
    )>,
    char_controller_output: Query<Option<&KinematicCharacterControllerOutput>, With<Character>>,
    mut commands: Commands,
) {
    let action_state = action_state_query.single();
    let Ok((entity, mut char_controller, mut sprite, char, mut currently_animating)) =
        player_query.get_single_mut()
    else {
        return;
    };

    let actions = action_state.get_pressed();

    if actions.is_empty() {
        trace!("No actions pressed.");
        *currently_animating = CurrentlyAnimating(false);
    } else {
        trace!("Moving character.");
    }

    let translation = &mut char_controller.translation.unwrap_or(GRAVITY);

    for action in actions {
        trace!("Action: {:#?}", action);
        match action {
            Action::Left => {
                translation.x = -char.movement_speed;
                sprite.flip_x = true;
                *currently_animating = CurrentlyAnimating(true);
            }
            Action::Right => {
                translation.x = char.movement_speed;
                sprite.flip_x = false;
                *currently_animating = CurrentlyAnimating(true);
            }
            Action::Jump => {
                if let Some(output) = char_controller_output.single() {
                    if output.grounded {
                        info!("Character is grounded, starting jump.");
                        *currently_animating = CurrentlyAnimating(false);
                        commands.entity(entity).insert(player::jump_component());
                    } else {
                        debug!("Character is not grounded, can't jump.");
                    }
                } else {
                    warn!("No character controller output found, can't jump.")
                }
            }
            Action::Grapple => { /* Do nothing, this is handled elsewhere. */ }
        }
    }

    char_controller.translation = Some(*translation);
}

/// Add a force to the player in the given direction (to be used for grappling).
pub fn add_grapple_force(
    mut player_query: Query<&mut KinematicCharacterController, With<Character>>,
    direction: Vec2,
) {
    let char_controller = &mut player_query.single_mut();

    // Completely replace player translation with grapple force
    let force = direction * 10.0;
    char_controller.translation = Some(force);
    trace!("Setting grapple force on player to: {:?}", force);
}

// LINT ALLOW: Ok because it's a bevy Query, and not actually very complex
#[allow(clippy::type_complexity)]
pub fn stop_jump(
    // ALLOW: pretty simple query
    mut char_query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<Character>, With<JumpComponent>),
    >,
    mut commands: Commands,
) {
    if let Ok((entity, char_controller_output)) = char_query.get_single_mut() {
        if char_controller_output.grounded {
            trace!("Character is grounded, stopping jump.");
            commands.entity(entity).remove::<JumpComponent>();
        }
    }
}
