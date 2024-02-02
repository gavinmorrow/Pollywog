use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{damage::Damage, health::Health, jump::JumpComponent, npc_movement::NpcMovement},
    level::ImageHandles,
};

use super::player;

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
    damage: Damage,
    rigid_body: RigidBody,
    npc_movement: NpcMovement,
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
            enemy: Enemy,
            health: Health::full(INITIAL_HEALTH),
            active_events: ActiveEvents::COLLISION_EVENTS,
            damage: Damage(player::INITIAL_HEALTH),
            rigid_body: RigidBody::Dynamic,
            npc_movement: NpcMovement { update },
            // velocity: Velocity {
            //     linvel: Vec2::new(64.0, 0.0),
            //     ..default()
            // },
            char_controller: KinematicCharacterController {
                translation: Some(Vec2::new(64.0, 0.0)),
                ..default()
            },
        }
    }
}

#[derive(Component)]
struct Enemy;

fn update(char: &mut KinematicCharacterController, pos: &GlobalTransform) {
    // let translation = &mut char.translation.unwrap_or_default();

    // translation.x = 1.0 * translation.x.signum();

    // // FIXME: why does this happen. fix it. this is hacky.
    // if pos.translation().x == 0.0 {
    //     info!(
    //         "so uhhh somehow the position (global transform)\
    //         of the enemy is ummmm *checks notes* `0.0`. how tf\
    //         is that possible. i'm just gonna return early dw."
    //     );
    //     return;
    // }

    // if pos.translation().x < 64.0 * 6.0 {
    //     *translation = Vec2::new(64.0, 0.0);
    // }
    // if pos.translation().x > 64.0 * 10.0 {
    //     *translation = Vec2::new(-64.0, 0.0);
    // }

    // char.translation = Some(*translation);
}
