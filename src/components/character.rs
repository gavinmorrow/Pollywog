use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{state::GameState, GRAVITY};

use super::jump::{jump, JumpComponent};

pub mod grapple;

#[derive(Default)]
pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::components::character::grapple::GrapplePlugin::default())
            // FIXME: maybe move the jump system somewhere else
            .add_systems(
                Update,
                (
                    r#move,
                    // Must go after so that the player gets moved immediately after
                    // the jump starts
                    jump.after(r#move),
                    // Must go before so that the player is off the ground when we check
                    stop_jump.before(r#move),
                )
                    .run_if(state_exists_and_equals(GameState::InGame)),
            );
    }
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
    mut player_query: Query<(&mut KinematicCharacterController, &mut Sprite, &Character)>,
    char_controller_output: Query<Option<&KinematicCharacterControllerOutput>, With<Character>>,
    mut jump_component_query: Query<&mut JumpComponent, With<Character>>,
) {
    let action_state = action_state_query.single();
    let Ok((mut char_controller, mut sprite, char)) = player_query.get_single_mut() else {
        return;
    };

    let actions = action_state.get_pressed();

    if !actions.is_empty() {
        trace!("Moving character.");
    }

    let translation = &mut char_controller.translation.unwrap_or(GRAVITY);

    for action in actions {
        trace!("Action: {:#?}", action);
        match action {
            Action::Left => {
                translation.x = -char.movement_speed;
                sprite.flip_x = true;
            }
            Action::Right => {
                translation.x = char.movement_speed;
                sprite.flip_x = false;
            }
            Action::Jump => match char_controller_output.single() {
                Some(output) => {
                    if output.grounded {
                        trace!("Character is grounded, starting jump.");
                        let mut jump_component = jump_component_query.single_mut();
                        jump_component.start_jump();
                    } else {
                        trace!("Character is not grounded, can't jump.");
                    }
                }
                None => trace!("No character controller output found, can't jump."),
            },
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

pub fn stop_jump(
    mut char_query: Query<
        (&mut JumpComponent, &KinematicCharacterControllerOutput),
        With<Character>,
    >,
) {
    let Ok((mut jump_component, char_controller_output)) = char_query.get_single_mut() else {
        return;
    };

    if char_controller_output.grounded && jump_component.is_jumping() {
        trace!("Player is grounded, stopping jump.");
        jump_component.stop_jump();
    }
}
