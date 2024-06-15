use crate::{
    components::{
        animated_sprite::{AnimatedSprite, AnimationIndices},
        character::{Action, Character},
        collect_coin::CoinCollector,
    },
    level,
    state::GameState,
    z_index,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{health::Health, jump::JumpComponent},
    GRAVITY,
};

const SIZE: Vec2 = Vec2::new(223.0, 373.0);

const JUMP_MAGNITUDE: Vec2 = Vec2::new(0.0, 10.0);
const MOVEMENT_SPEED: f32 = 3.0;

const TEXTURE_PATH: &str = "player_new.atlas.png";

pub const INITIAL_HEALTH: f32 = 100.0;

pub fn player_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn);
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    animation: AnimatedSprite,
    character_controller: KinematicCharacterController,
    collider: Collider,
    player: Player,
    damping: Damping,
    input_manager: InputManagerBundle<Action>,
    external_force: ExternalForce,
    gravity_scale: GravityScale,
    jump_component: JumpComponent,
    char: Character,
    health: Health,
    active_events: ActiveEvents,
    rigid_body: RigidBody,
    coins: CoinCollector,
}

impl PlayerBundle {
    fn new(
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        window: &Window,
    ) -> Self {
        debug!("Creating player bundle");

        let layout = TextureAtlasLayout::from_grid(SIZE, 5, 2, None, None);
        let layout = texture_atlas_layouts.add(layout);

        let texture = asset_server.load(TEXTURE_PATH);
        let animation_indices = AnimationIndices { first: 0, last: 8 };

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, window.height(), z_index::LEVEL_BASE),
                    ..default()
                },
                texture,
                sprite: Sprite {
                    custom_size: Some(SIZE),
                    ..default()
                },
                ..default()
            },
            animation: AnimatedSprite::new(layout, animation_indices),
            // FIXME: maybe not default? just trying to get this to work for now
            character_controller: KinematicCharacterController {
                translation: Some(GRAVITY),
                ..default()
            },
            collider: Collider::cuboid(SIZE.x / 2.0, SIZE.y / 2.0),
            player: Player,
            damping: Damping {
                angular_damping: 3.0,
                linear_damping: 0.0,
            },
            input_manager: InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: get_input_map(),
            },
            external_force: ExternalForce::default(),
            gravity_scale: GravityScale(1.0),
            jump_component: JumpComponent::new(JUMP_MAGNITUDE, false),
            char: Character {
                movement_speed: MOVEMENT_SPEED,
            },
            health: Health::full(INITIAL_HEALTH),
            active_events: ActiveEvents::COLLISION_EVENTS,
            rigid_body: RigidBody::KinematicPositionBased,
            coins: CoinCollector::default(),
        }
    }
}

fn get_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();

    input_map
        .insert(Action::Left, KeyCode::ArrowLeft)
        .insert(Action::Left, KeyCode::KeyA)
        .insert(Action::Right, KeyCode::ArrowRight)
        .insert(Action::Right, KeyCode::KeyD)
        .insert(Action::Jump, KeyCode::ArrowUp)
        .insert(Action::Jump, KeyCode::KeyW)
        .insert(Action::Jump, KeyCode::Space)
        .insert(Action::Grapple, KeyCode::KeyE)
        .insert(Action::Grapple, KeyCode::Slash);

    input_map
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    debug!("Spawning player");
    level::spawn_entity(
        &mut commands,
        PlayerBundle::new(asset_server, texture_atlas_layouts, window),
    );
}
