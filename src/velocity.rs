use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

pub fn update_velocity(mut query: Query<(&Velocity, &mut Transform)>) {
    trace!("Applying velocity to {} entities", query.iter().len());
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0;
    }
}
