use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;

const SIZE: f32 = 64.0;
const SIZE_VEC2: Vec2 = Vec2::new(SIZE, SIZE);

const TEXTURE_PATH: &str = "player.png";

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    collider: Collider,
    player: Player,
    angular_damping: AngularDamping,
    linear_damping: LinearDamping,
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
                    custom_size: Some(SIZE_VEC2),
                    ..default()
                },
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(SIZE / 2.0),
            player: Player,
            angular_damping: AngularDamping(3.0),
            linear_damping: LinearDamping(1.0),
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

// FIXME: make a better movement system, this is just a placeholder
pub fn r#move(key_input: Res<Input<KeyCode>>, mut query: Query<&mut LinearVelocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        for key in key_input.get_pressed() {
            match key {
                KeyCode::Left | KeyCode::A => velocity.x = -300.0,
                KeyCode::Right | KeyCode::D => velocity.x = 300.0,
                _ => {}
            }
        }
    }
}
