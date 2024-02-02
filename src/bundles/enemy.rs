use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{damage::Damage, health::Health, jump::JumpComponent},
    level::ImageHandles,
};

use super::player;

const SIZE: f32 = 64.0;
const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

const JUMP_MAGNITUDE: Vec2 = Vec2::new(0.0, 10.0);
const SPEED: Vec2 = Vec2::new(1.0, 0.0);

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
    damage: Damage,
    rigid_body: RigidBody,
    // velocity: Velocity,
    char_controller: KinematicCharacterController,
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
            enemy: Enemy::default(),
            health: Health::full(INITIAL_HEALTH),
            active_events: ActiveEvents::COLLISION_EVENTS,
            damage: Damage(player::INITIAL_HEALTH),
            rigid_body: RigidBody::Dynamic,
            // velocity: Velocity {
            //     linvel: Vec2::new(64.0, 0.0),
            //     ..default()
            // },
            char_controller: KinematicCharacterController {
                translation: Some(SPEED),
                ..default()
            },
        }
    }
}

#[derive(Component, Default)]
pub struct Enemy {
    direction: Direction,
    speed: Vec2,
}

// impl Enemy {
//     fn direction(&self) -> Direction {
//         // self.direction
//         if self.speed.x < 0.0 {
//             Direction::Left
//         } else {
//             Direction::Right
//         }
//     }
// }

#[derive(Default)]
enum Direction {
    Left,
    #[default]
    Right,
}

pub fn move_enemy(mut enemies: Query<(&mut KinematicCharacterController, &Enemy)>) {
    for (mut char, enemy) in enemies.iter_mut() {
        char.translation = Some(match enemy.direction {
            Direction::Left => enemy.speed * Vec2::new(-1.0, 0.0),
            Direction::Right => enemy.speed,
        });
    }
}

pub fn enemy_sprite_flipped(mut enemies: Query<(&mut Sprite, &Enemy)>) {
    for (mut sprite, enemy) in enemies.iter_mut() {
        sprite.flip_x = match enemy.direction {
            Direction::Left => true,
            Direction::Right => false,
        };
    }
}

pub fn swap_direction(mut enemies: Query<(&mut Enemy, &GlobalTransform)>) {
    for (mut enemy, pos) in enemies.iter_mut() {
        let pos = pos.translation().x;
        if pos < 64.0 * 6.0 {
            enemy.direction = Direction::Right;
        } else if pos > 64.0 * 8.0 {
            enemy.direction = Direction::Left;
        }
    }
}
