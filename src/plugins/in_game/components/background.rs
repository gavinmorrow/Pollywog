use bevy::prelude::*;

use super::super::{bundles::background::Background, InGameSet};

pub fn background_plugin(app: &mut App) {
    app.add_systems(Update, parallax.in_set(InGameSet));
}

fn parallax(
    camera: Query<&Transform, With<Camera>>,
    mut backgrounds: Query<(&mut Transform, &Background), Without<Camera>>,
) {
    let camera = camera.single();
    for (mut transform, background) in &mut backgrounds {
        transform.translation.x = background.new_position(camera.translation.x);
    }
}
