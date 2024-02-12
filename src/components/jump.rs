use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GRAVITY;

#[derive(Component, Clone, Default)]
pub struct JumpComponent {
    magnitude: Vec2,
    velocity: Vec2,
    jumping: bool,
}

#[allow(dead_code)] // FIXME: Remove this (when more things that jump exist)
impl JumpComponent {
    pub fn new(magnitude: Vec2, jumping: bool) -> Self {
        JumpComponent {
            magnitude,
            velocity: Vec2::ZERO,
            jumping,
        }
    }

    pub fn start_jump(&mut self) {
        self.velocity = self.magnitude;
        self.jumping = true;
    }

    // TODO: make this trigger automatically when the character hits the ground
    pub fn stop_jump(&mut self) {
        self.jumping = false;
    }

    pub fn is_jumping(&self) -> bool {
        self.jumping
    }

    pub fn magnitude(&self) -> Vec2 {
        self.magnitude
    }
}

pub fn jump(
    mut jump_component_query: Query<(&mut JumpComponent, &mut KinematicCharacterController)>,
) {
    for (mut jump_component, mut kinematic_character_controller) in jump_component_query.iter_mut()
    {
        if !jump_component.jumping {
            continue;
        }

        trace!("Jumping with velocity {:?}", jump_component.velocity);

        // Apply force to character controller
        let translation = kinematic_character_controller
            .translation
            .unwrap_or_default();
        let translation = Vec2::new(translation.x, 0.0) + GRAVITY;
        debug!(
            "{} {} {}",
            translation,
            jump_component.velocity,
            translation + jump_component.velocity
        );
        kinematic_character_controller.translation = Some(translation + jump_component.velocity);
    }
}
