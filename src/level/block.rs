use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const SIZE: f32 = 64.0;
pub const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

#[derive(Component, Clone, Default)]
pub struct Block;

#[derive(Bundle, Clone)]
pub struct BlockBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    collider: Collider,
    friction: Friction,
    block: Block,
}

#[derive(Component, Clone, Default)]
pub struct JumpCollisionBox;

#[derive(Bundle, Clone)]
pub struct JumpCollisionBoxBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    jump_collision_box: JumpCollisionBox,
}

impl core::fmt::Debug for BlockBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(
            format!(
                "BlockBundle (transform: {:#?})",
                self.sprite_bundle.transform
            )
            .as_str(),
        )
        .finish()
    }
}

impl BlockBundle {
    pub fn new(translation: Vec3) -> Self {
        trace!("Creating block bundle (translation: {:?})", translation);

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation,
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(SIZE_VEC2),
                    color: Color::rgb(0.22, 0.14, 0.07),
                    ..default()
                },
                ..default()
            },
            // FIXME: Maybe use a kinematic rigid body instead?
            //
            // Doing this for now because I'm just trying to get the game
            // working and we don't have any moving blocks yet
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(SIZE, SIZE),
            friction: Friction::new(1.0),
            block: Block,
        }
    }
}

impl JumpCollisionBoxBundle {
    pub fn new(translation: Vec3) -> Self {
        // Add padding on left so wall clinging doesn't work
        let translation = translation + Vec3::new(1.0, 0.0, 0.0);

        trace!(
            "Creating jump collision box bundle (translation: {:?})",
            translation
        );

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            // Add padding so wall clinging doesn't work
            collider: Collider::cuboid(SIZE - 2.0, SIZE - 2.0),
            jump_collision_box: JumpCollisionBox,
        }
    }
}
