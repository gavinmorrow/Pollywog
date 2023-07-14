use bevy::prelude::*;

pub const SIZE_F32: f32 = 64.0;
pub const SIZE: Vec2 = Vec2::new(SIZE_F32, SIZE_F32);

#[derive(Bundle, Clone)]
pub struct BlockBundle {
    sprite_bundle: SpriteBundle,
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
                    custom_size: Some(SIZE),
                    color: Color::rgb(0.22, 0.14, 0.07),
                    ..default()
                },
                ..default()
            },
        }
    }
}
