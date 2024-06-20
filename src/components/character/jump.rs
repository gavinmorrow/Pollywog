use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GRAVITY;

#[derive(Component, Clone, Default)]
pub struct JumpComponent {
    pub magnitude: f32,
    pub timer: Timer,
}

impl JumpComponent {
    pub fn velocity_at(&self, elapsed: Duration) -> f32 {
        let x = elapsed.as_secs_f32();
        let d = self.timer.duration().as_secs_f32();
        let m = self.magnitude;

        let half = d / 2.0;
        // let s = if x < half { 1.0 } else { -1.0 };
        let s = -1.0;

        m * s * 2.0 * ((1.0 / d) * x - 0.5).tan().powi(1) + GRAVITY.y
    }
}

pub fn jump(
    time: Res<Time>,
    mut jump_component_query: Query<(&mut JumpComponent, &mut KinematicCharacterController)>,
) {
    for (mut jump_component, mut char_controller) in &mut jump_component_query {
        jump_component.timer.tick(time.delta());
        let vel = jump_component.velocity_at(jump_component.timer.elapsed());
        debug!("Jumping w/ vel {}", vel);
        char_controller.translation.as_mut().map(|t| t.y = vel);
    }
}
