use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::GRAVITY;

use super::jump::JumpComponent;

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
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &mut Sprite,
        &Character,
    )>,
    mut jump_component_query: Query<&mut JumpComponent, With<Character>>,
) {
    let action_state = action_state_query.single();
    let Ok((mut char_controller, char_controller_output, mut sprite, char)) =
        player_query.get_single_mut()
    else {
        return;
    };

    let actions = action_state.get_pressed();

    if !actions.is_empty() {
        trace!("Moving character.");
    }

    char_controller.translation = Some(char_controller.translation.unwrap_or(GRAVITY));
    let translation = &mut char_controller
        .translation
        .expect("Just set to a Some value above.");

    let movement_speed = char.movement_speed;

    for action in actions {
        trace!("Action: {:#?}", action);
        match action {
            Action::Left => {
                translation.x = -movement_speed;
                sprite.flip_x = true;
            }
            Action::Right => {
                translation.x = movement_speed;
                sprite.flip_x = false;
            }
            Action::Jump => {
                if char_controller_output.grounded {
                    trace!("Character is grounded, starting jump.");
                    let mut jump_component = jump_component_query.single_mut();
                    jump_component.start_jump();
                } else {
                    trace!("Character is not grounded, can't jump.");
                }
            }
            Action::Grapple => { /* Do nothing, this is handled elsewhere. */ }
        }
    }

    char_controller.translation = Some(*translation);
}
