use bevy::prelude::*;

use crate::velocity::Velocity;

/// The acceleration to apply to entities with a `Gravity` component (each physics update).
// FIXME: tweak this value to make the game feel better
const GRAVITY: Vec3 = Vec3::new(0.0, -0.01, 0.0);

#[derive(Component)]
pub struct Gravity;

pub fn apply_gravity(mut query: Query<&mut Velocity, With<Gravity>>) {
    trace!("Applying gravity to {} entities", query.iter().len());
    for mut velocity in query.iter_mut() {
        velocity.0 += GRAVITY;
    }
}
