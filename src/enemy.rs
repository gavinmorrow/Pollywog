use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{
        health::Health, jump::JumpComponent, kills_player::KillsPlayerComponent,
        npc_movement::NpcMovement,
    },
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
    npc_movement: NpcMovement,
    velocity: Velocity,
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
            npc_movement: NpcMovement {
                modify_vel: |vel, pos| {
                    vel.linvel.x = 64.0 * vel.linvel.x.signum();
                    if pos.translation().x < 64.0 * 6.0 || pos.translation().x > 64.0 * 10.0 {
                        vel.linvel *= -1.0;
                        vel.angvel *= -1.0;
                    }
                },
            },
            velocity: Velocity {
                linvel: Vec2::new(64.0, 0.0),
                ..default()
            },
        }
    }
}

#[derive(Component)]
struct Enemy;
