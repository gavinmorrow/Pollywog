use bevy::prelude::*;

use crate::{gravity::Gravity, velocity::Velocity};

const SIZE_F32: f32 = 64.0;
const SIZE: Vec2 = Vec2::new(SIZE_F32, SIZE_F32);

const TEXTURE_PATH: &str = "player.png";

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    gravity: Gravity,
}

impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>, window: &Window) -> Self {
        debug!("Creating player bundle");

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, window.height(), 0.0),
                    ..default()
                },
                texture: asset_server.load(TEXTURE_PATH),
                sprite: Sprite {
                    custom_size: Some(SIZE),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity::new(00.0, 0.0, 0.0),
            gravity: Gravity {},
        }
    }
}
