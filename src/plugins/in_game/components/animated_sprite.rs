use bevy::prelude::*;

pub fn animated_sprite_plugin(app: &mut App) {
    app.add_systems(Update, animate_sprite);
}

// Derived from https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs

#[derive(Bundle, Default)]
pub struct AnimatedSprite {
    pub texture_atlas: TextureAtlas,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
}

#[derive(Component, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    pub fn next(&self, index: usize) -> usize {
        let index = index + 1;
        if index >= self.last {
            self.first
        } else {
            index
        }
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// Make the sprite animate
#[derive(Component)]
pub struct CurrentlyAnimating;

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas),
        With<CurrentlyAnimating>,
    >,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = indices.next(atlas.index);
        }
    }
}
