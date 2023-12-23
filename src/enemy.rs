use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::jump::JumpComponent;

const SIZE: f32 = 64.0;
const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

const JUMP_MAGNITUDE: Vec2 = Vec2::new(0.0, 10.0);

const TEXTURE_PATH: &str = "enemy.png";

#[derive(Bundle)]
pub struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    jump_component: JumpComponent,
    enemy: Enemy,
}

impl EnemyBundle {
    pub fn new(translation: Vec2, asset_server: &Res<AssetServer>) -> Self {
        EnemyBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(SIZE_VEC2),
                    ..default()
                },
                transform: Transform {
                    translation: translation.extend(0.0),
                    ..default()
                },
                texture: asset_server.load(TEXTURE_PATH),
                ..default()
            },
            collider: Collider::cuboid(SIZE / 2.0, SIZE / 2.0),
            jump_component: JumpComponent::new(JUMP_MAGNITUDE, false),
            enemy: Enemy {},
        }
    }
}

#[derive(Component)]
struct Enemy {}
