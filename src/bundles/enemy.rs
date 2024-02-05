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
const SPEED: Vec2 = Vec2::new(2.0, 0.0);

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
            enemy: Enemy::new(SPEED, 7.0, 12.0),
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

#[derive(Component)]
pub struct Enemy {
    direction: Direction,
    speed: Vec2,
    min_speed: Vec2,
    left_boundary: f32,
    right_boundary: f32,
}

impl Enemy {
    pub fn new(speed: Vec2, left_boundary: f32, right_boundary: f32) -> Enemy {
        Enemy {
            direction: Direction::Right,
            speed,
            min_speed: Vec2::new(0.1, 0.0),
            left_boundary,
            right_boundary,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn signum(&self) -> f32 {
        match self {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        }
    }
}

pub fn move_enemy(mut enemies: Query<(&mut KinematicCharacterController, &Enemy)>) {
    for (mut char, enemy) in enemies.iter_mut() {
        char.translation = Some(enemy.speed);
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

pub fn swap_direction(mut enemies: Query<(&mut Enemy, &Transform)>) {
    for (mut enemy, pos) in enemies.iter_mut() {
        let pos = pos.translation.x;

        let left_boundary = enemy.left_boundary * 64.0;
        let right_boundary = enemy.right_boundary * 64.0;
        if pos <= left_boundary {
            trace!("Enemy past left boundary. Swapping direction.");
            enemy.direction = Direction::Right;
        } else if pos >= right_boundary {
            trace!("Enemy past right boundary. Swapping direction.");
            enemy.direction = Direction::Left;
        }

        let total_dist = right_boundary - left_boundary;

        let relative_x = pos - left_boundary; // relative to left boundary
        let percent = relative_x / total_dist;

        let rel_percent = match enemy.direction {
            Direction::Left => 1.0 - percent,
            Direction::Right => percent,
        };

        // x needs to be in the range 0.0-0.5 going towards the center,
        // and then 0.5-0.0 going away from the center.
        //
        // So, the 0.0-0.5 range is ok as is, but we need to map the 0.5-1 range to 0.5-0.
        let x = if rel_percent < 0.5 {
            rel_percent
        } else {
            // First move 0.5-1 to 0-0.5.
            // Then, flip it.
            0.5 - (rel_percent - 0.5)
        };

        let scale_factor = 2.0 * enemy.direction.signum();
        let x = x.max(0.0).sqrt() + enemy.min_speed.x;
        enemy.speed.x = x * scale_factor;
    }
}
