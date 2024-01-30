use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{health::Health, jump::JumpComponent, kills_player::KillsPlayerComponent},
    level::ImageHandles,
};

const SIZE: f32 = 64.0;
const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

const JUMP_MAGNITUDE: Vec2 = Vec2::new(0.0, 10.0);

pub const TEXTURE_PATH: &str = "enemy.png";

const INITIAL_HEALTH: f32 = 100.0;

#[derive(Bundle)]
pub struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    jump_component: JumpComponent,
    enemy: Enemy,
    health: Health,
    active_events: ActiveEvents,
    kills_player: KillsPlayerComponent,
    rigid_body: RigidBody,
}

impl EnemyBundle {
    pub fn new(translation: Vec2, handles: &ImageHandles) -> Self {
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
                texture: handles.texture.clone(),
                ..default()
            },
            collider: Collider::ball(SIZE / 2.0),
            jump_component: JumpComponent::new(JUMP_MAGNITUDE, false),
            enemy: Enemy,
            health: Health::full(INITIAL_HEALTH),
            active_events: ActiveEvents::COLLISION_EVENTS,
            kills_player: KillsPlayerComponent,
            rigid_body: RigidBody::Dynamic,
        }
    }
}

#[derive(Component)]
struct Enemy;
