use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::z_index;

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Block;

#[derive(Bundle, Clone)]
pub struct BlockBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    collider: Collider,
    friction: Friction,
    block: Block,
}

impl BlockBundle {
    pub fn new(translation: Vec2) -> Self {
        trace!("Creating block bundle (translation: {:?})", translation);

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: translation.extend(z_index::LEVEL_BASE),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    color: Color::rgb(0.22, 0.14, 0.07),
                    ..default()
                },
                ..default()
            },
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(32.0, 32.0),
            friction: Friction {
                coefficient: 0.5,
                ..default()
            },
            block: Block,
        }
    }
}
