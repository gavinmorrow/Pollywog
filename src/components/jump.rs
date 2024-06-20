use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GRAVITY;

#[derive(Component, Clone, Default)]
pub struct JumpComponent {
    pub magnitude: Vec2,
    pub total_time: Duration,
    pub timer: Timer,
}

impl JumpComponent {
    pub fn velocity_at(&self, duration: Duration) -> f32 {
        -3.0 * (duration.as_secs_f32() - 0.5).tan().powi(3) + 0.5
    }
}

pub fn jump(
    time: Res<Time>,
    mut jump_component_query: Query<(&mut JumpComponent, &mut KinematicCharacterController)>,
) {
    for (mut jump_component, mut char_controller) in &mut jump_component_query {
        jump_component.timer.tick(time.delta());
        char_controller
            .translation
            .as_mut()
            .map(|t| t.y = jump_component.velocity_at(jump_component.timer.duration()));
    }
}
