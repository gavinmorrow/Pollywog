//! Based off of <https://www.youtube.com/watch?v=hG9SzQxaCm8> and
//! <https://www.youtube.com/watch?v=eeLPL3Y9jjA>.

use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use super::Character;

#[derive(Component, Debug, Clone, Default)]
pub struct JumpComponent {
    gravity: f32,
    initial_velocity: f32,

    timer: Timer,
}

impl JumpComponent {
    pub fn new(height: f32, time_to_peak: f32) -> Self {
        Self {
            gravity: -2.0 * height / (time_to_peak * time_to_peak),
            initial_velocity: 2.0 * height / time_to_peak,

            timer: Timer::from_seconds(time_to_peak * 2.0, TimerMode::Once),
        }
    }

    pub fn velocity_at(&self, elapsed: Duration) -> f32 {
        self.gravity * elapsed.as_secs_f32() + self.initial_velocity
    }
}

pub fn jump(
    time: Res<Time>,
    mut jump_component_query: Query<(&mut JumpComponent, &mut KinematicCharacterController)>,
) {
    for (mut jump_component, mut char_controller) in &mut jump_component_query {
        // Tick timer
        jump_component.timer.tick(time.delta());

        // Calculate velocity
        let elapsed = jump_component.timer.elapsed();
        let vel = jump_component.velocity_at(elapsed);
        debug!("Jumping w/ vel {}", vel);

        // Apply velocity
        let mut trans = char_controller.translation.unwrap_or_default();
        trans.y = vel;
        char_controller.translation = Some(trans);
    }
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
            info!("Character is grounded, stopping jump.");
            commands.entity(entity).remove::<JumpComponent>();
        }
    }
}
