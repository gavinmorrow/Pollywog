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

        m * -2.0 * ((1.0 / d) * x - 0.5).tan().powi(1) + GRAVITY.y
    }
}

pub fn jump(
    time: Res<Time>,
    mut jump_component_query: Query<(&mut JumpComponent, &mut KinematicCharacterController)>,
) {
    for (mut jump_component, mut char_controller) in &mut jump_component_query {
        jump_component.timer.tick(time.delta());

        let vel = jump_component.velocity_at(jump_component.timer.elapsed());
        char_controller.translation = Some(char_controller.translation.unwrap_or_default());
        let t = char_controller.translation.as_mut().unwrap();
        debug!("Jumping w/ vel {}", vel);
        t.y = vel;
    }
}
