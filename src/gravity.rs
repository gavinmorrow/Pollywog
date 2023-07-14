use bevy::prelude::*;

use crate::velocity::Velocity;

/// The acceleration to apply to entities with a `Gravity` component (per second).
// FIXME: tweak this value to make the game feel better
const GRAVITY: Vec3 = Vec3::new(0.0, -100.0, 0.0);

#[derive(Component)]
pub struct Gravity;

pub fn apply_gravity(mut query: Query<&mut Velocity, With<Gravity>>, time_step: Res<FixedTime>) {
    trace!("Applying gravity to {} entities", query.iter().len());
    for mut velocity in query.iter_mut() {
        velocity.0 += GRAVITY * time_step.period.as_secs_f32();
    }
}
