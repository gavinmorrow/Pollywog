use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::animated_sprite::{
        AnimatedSprite, AnimationIndices, AnimationTimer, CurrentlyAnimating,
    },
    z_index,
};

// FIXME: when this is 64.0, the player and enemy get stuck underneath it
//        and the player for some reason doesn't collect it??? very weird
const SIZE: f32 = 60.0;

const TEXTURE_SIZE: Vec2 = Vec2::new(79.0, 79.0);
pub const TEXTURE_PATH: &str = "coin.atlas.png";

#[derive(Component, Default)]
pub struct Coin;

#[derive(Bundle)]
pub struct CoinBundle {
    collider: Collider,
    sensor: Sensor,
    active_collision_types: ActiveCollisionTypes,

    animation: AnimatedSprite,
    currently_animating: CurrentlyAnimating,
    sprite_bundle: SpriteBundle,

    coin: Coin,
}

impl CoinBundle {
    pub fn new(
        translation: Vec2,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let layout = TextureAtlasLayout::from_grid(TEXTURE_SIZE, 7, 2, None, None);
        let layout = texture_atlas_layouts.add(layout);

        let texture = asset_server.load(TEXTURE_PATH);
        let animation_indices = AnimationIndices { first: 0, last: 12 };

        CoinBundle {
            collider: Collider::ball(SIZE / 2.0),
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_STATIC,

            animation: AnimatedSprite {
                texture_atlas: TextureAtlas {
                    layout,
                    index: animation_indices.first,
                },
                animation_indices,
                animation_timer: AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
                ..default()
            },
            currently_animating: CurrentlyAnimating,
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform {
                    translation: translation.extend(z_index::LEVEL_BASE),
                    scale: Vec3::splat(SIZE / TEXTURE_SIZE.x),
                    ..default()
                },
                ..default()
            },

            coin: Coin,
        }
    }
}
