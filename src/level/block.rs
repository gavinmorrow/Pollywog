use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Block;

#[derive(Bundle, Clone)]
pub struct BlockBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    collider: Collider,
    friction: Friction,
    block: Block,
    // active_events: ActiveEvents,
}

impl BlockBundle {
    pub fn new(translation: Vec2) -> Self {
        // Needed because `SpriteBundle` requires a `Vec3`
        let translation = translation.extend(0.0);

        trace!("Creating block bundle (translation: {:?})", translation);

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation,
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
            // active_events: ActiveEvents::default(),
        }
    }
}
