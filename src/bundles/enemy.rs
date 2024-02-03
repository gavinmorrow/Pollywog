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

#[derive(Component)]
pub struct Enemy {
    direction: Direction,
    speed: Vec2,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            direction: Direction::default(),
            speed: Vec2::default(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
enum Direction {
    Left,
    #[default]
    Right,
}

impl Direction {
    fn flip(&mut self) {
        *self = match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
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

        let left_boundary = 64.0 * 7.0;
        let right_boundary = 64.0 * 8.0;

        let total_dist = right_boundary - left_boundary;

        let relative_x = pos - left_boundary; // relative to left boundary
        let percent = match enemy.direction {
            // Reverse the percent for the other direction to make later code simpler
            Direction::Left => 1.0 - relative_x / total_dist,
            Direction::Right => relative_x / total_dist,
        };

        eprintln!("\np: {}", percent);
        eprintln!("b: {}", enemy.speed);

        if percent > 0.75 {
            // Slow done the enemy as it approaches the edge
            let scale_factor = (percent - 0.75) / 10.0;
            enemy.speed.x -= SPEED.x * scale_factor;
        } else if percent < 0.25 {
            // Speed up the enemy if it's leaving the edge
            let scale_factor = percent.abs() / 10.0;
            enemy.speed.x += SPEED.x * scale_factor;
        } else {
            // Reset the speed if the enemy is in the middle
            enemy.speed.x = SPEED.x * enemy.speed.x.signum();
        }

        if percent == 0.0 {
            enemy.speed.x = SPEED.x;
        }

        enemy.speed.x = enemy.speed.x.min(SPEED.x);

        eprintln!("a: {}", enemy.speed);
    }
}
