use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub const SIZE: f32 = 64.0;
pub const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

#[derive(Bundle, Clone)]
pub struct BlockBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    collider: Collider,
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
            rigid_body: RigidBody::Static,
            collider: Collider::cuboid(SIZE, SIZE),
        }
    }
}
