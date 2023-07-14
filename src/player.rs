use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;

const SIZE_F32: f32 = 64.0;
const SIZE: Vec2 = Vec2::new(SIZE_F32, SIZE_F32);

const TEXTURE_PATH: &str = "player.png";

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    collider: Collider,
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
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(SIZE / 2.0),
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    debug!("Spawning player");
    commands.spawn(PlayerBundle::new(asset_server, window));
}
