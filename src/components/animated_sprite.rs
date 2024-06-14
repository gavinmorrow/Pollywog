use bevy::prelude::*;

pub fn animated_sprite_plugin(app: &mut App) {
    app.add_systems(Update, animate_sprite);
}

// Derived from https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs

#[derive(Component)]
pub struct AnimatedSprite {
    texture_atlas: TextureAtlas,
    animation_indices: AnimationIndices,
    timer: AnimationTimer,
}

impl AnimatedSprite {
    pub fn new(
        texture_atlas_layout: Handle<TextureAtlasLayout>,
        animation_indices: AnimationIndices,
    ) -> Self {
        AnimatedSprite {
            texture_atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
