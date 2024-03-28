use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::level::ImageHandles;

// FIXME: when this is 64.0, the player and enemy get stuck underneath it
//        and the player for some reason doesn't collect it??? very weird
const SIZE: f32 = 60.0;
pub const TEXTURE_PATH: &str = "coin.png";

#[derive(Component, Default)]
pub struct Coin;

#[derive(Bundle)]
pub struct CoinBundle {
    collider: Collider,
    sensor: Sensor,
    active_collision_types: ActiveCollisionTypes,
    sprite_bundle: SpriteBundle,
    coin: Coin,
}

impl CoinBundle {
    pub fn new(translation: Vec2, handles: &ImageHandles) -> Self {
        CoinBundle {
            collider: Collider::ball(SIZE / 2.0),
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_STATIC,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(SIZE, SIZE)),
                    ..default()
                },
                texture: handles.texture.clone(),
                transform: Transform::from_translation(translation.extend(0.0)),
                ..default()
            },
            coin: Coin,
        }
    }
}
