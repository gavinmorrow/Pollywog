use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GRAVITY;

#[derive(Component, Clone, Default)]
pub struct JumpComponent {
    magnitude: Vec2,
    velocity: Vec2,
    jumping: bool,
}

impl JumpComponent {
    pub fn new(magnitude: Vec2, jumping: bool) -> Self {
        JumpComponent {
            magnitude,
            velocity: Vec2::ZERO,
            jumping,
        }
    }

    pub fn apply_gravity(&mut self) {
        // FIXME: When self.magnitude contains a -15.0, that coordinate gets
        // turned into NaN (bc of division by zero)
        self.velocity += GRAVITY / (Vec2::new(15.0, 15.0) + self.magnitude);
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

    #[allow(dead_code)] // FIXME: Remove this (when more things that jump exist)
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
        let translation = Vec2::new(translation.x, 0.0);
        kinematic_character_controller.translation = Some(translation + jump_component.velocity);

        // Apply gravity to jump component
        jump_component.apply_gravity();
    }
}
