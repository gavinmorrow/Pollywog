use bevy::prelude::*;

/// Indicates that an entity should have a velocity applied to it.
#[derive(Component)]
pub struct Velocity(
    /// The velocity to apply to entities with a `Velocity` component (per second).
    pub Vec3,
);

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

pub fn update_velocity(mut query: Query<(&Velocity, &mut Transform)>, time_step: Res<FixedTime>) {
    trace!("Applying velocity to {} entities", query.iter().len());
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time_step.period.as_secs_f32();
    }
}
