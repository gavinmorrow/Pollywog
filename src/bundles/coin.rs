use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const SIZE: f32 = 32.0;

#[derive(Component, Default)]
pub struct Coin;

#[derive(Bundle)]
pub struct CoinBundle {
    collider: Collider,
    rigid_body: RigidBody,
    sprite_bundle: SpriteBundle,
    coin: Coin,
}

impl CoinBundle {
    pub fn new(translation: Vec2) -> Self {
        CoinBundle {
            collider: Collider::ball(SIZE / 2.0),
            rigid_body: RigidBody::KinematicVelocityBased,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(SIZE, SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(translation.extend(0.0)),
                ..default()
            },
            coin: Coin,
        }
    }
}
