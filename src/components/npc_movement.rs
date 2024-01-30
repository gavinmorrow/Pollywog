use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct NpcMovementPlugin;
impl Plugin for NpcMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_npcs);
    }
}

/// NOTE: must be used with a `Velocity` component on the same entity.
#[derive(Component)]
pub struct NpcMovement {
    pub modify_vel: fn(vel: &mut Velocity, pos: &GlobalTransform),
}

fn move_npcs(mut npcs: Query<(&mut Velocity, &GlobalTransform, &NpcMovement)>) {
    for (mut vel, transform, movement) in npcs.iter_mut() {
        (movement.modify_vel)(&mut vel, transform)
    }
}
