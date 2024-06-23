use std::collections::HashMap;

use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::{
    bundles::{
        background::{BackgroundBundle, BackgroundSection, Biome},
        coin::CoinBundle,
        enemy::EnemyBundle,
    },
    state::GameState,
};

use self::block::BlockBundle;

mod block;

const SIZE: f32 = 64.0;

pub fn level_plugin(app: &mut App) {
    app.init_state::<LevelState>()
        .insert_resource(GameAsset::default())
        .add_plugins(JsonAssetPlugin::<LevelAsset>::new(&["level.json"]))
        .add_systems(
            OnEnter(LevelState::LoadingAssets),
            (load_level_asset, load_image_assets),
        )
        .add_systems(
            Update,
            wait_for_level_load.run_if(in_state(LevelState::LoadingAssets)),
        )
        .add_systems(OnEnter(LevelState::ConstructingLevel), construct_level_res)
        .add_systems(
            Update,
            wait_for_level_start.run_if(in_state(LevelState::WaitingForLevelStart)),
        )
        .add_systems(OnEnter(LevelState::SpawningBlocks), spawn_blocks)
        .add_systems(OnExit(GameState::InGame), reprime_level_state);
}

fn load_level_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_handle = asset_server.load::<LevelAsset>("levels/hello_world.level.json");
    let level_handle = LevelHandle(level_handle);

    info!("Loading level asset: {:?}", level_handle);

    commands.insert_resource(level_handle);
}

fn wait_for_level_load(
    mut next_state: ResMut<NextState<LevelState>>,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<LevelAsset>>,
) {
    if level_assets.get(level_handle.0.clone()).is_some() {
        next_state.set(LevelState::ConstructingLevel);
    }
}

fn wait_for_level_start(
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    if game_state.get() == &GameState::InGame {
        info!("Finished waiting for level start, spawning blocks");
        next_state.set(LevelState::SpawningBlocks);
    }
}

fn construct_level_res(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelState>>,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<LevelAsset>>,
) {
    let level_asset = level_assets.get(level_handle.0.clone());
    let Some(level_asset) = level_asset.cloned() else {
        error!("Failed to load level asset: {:?}", level_handle);
        return;
    };
    let level = Level::from(level_asset);

    info!("Constructing level resource");

    commands.insert_resource(level);
    next_state.set(LevelState::WaitingForLevelStart);
}

fn load_image_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAsset>) {
    let mut handles = std::collections::HashMap::from([
        (
            ImageHandleId::Enemy,
            ImageHandles {
                texture: asset_server.load(crate::bundles::enemy::TEXTURE_PATH),
            },
        ),
        (
            ImageHandleId::Coin,
            ImageHandles {
                texture: asset_server.load(crate::bundles::coin::TEXTURE_PATH),
            },
        ),
    ]);

    BackgroundSection::enumerate()
        .iter()
        .map(|section| {
            (
                section.image_handle_id(),
                ImageHandles {
                    texture: asset_server.load(section.texture_path()),
                },
            )
        })
        .for_each(|(k, v)| {
            handles.insert(k, v);
        });

    game_assets.image_handles = handles;
}

fn spawn_blocks(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelState>>,
    level: Res<Level>,
    game_assets: Res<GameAsset>,
    // FIXME: This is a hack to get the asset server in the system.
    //        Maybe take out all the preemptive loading and just load on demand? (like the player)
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Spawning blocks for level: {}", level.name);

    for section in level.biome.sections() {
        spawn_entity(
            &mut commands,
            BackgroundBundle::new(
                section,
                game_assets
                    .image_handles
                    .get(&section.image_handle_id())
                    .expect("background assets must be loaded"),
            ),
        )
    }

    for block in &level.blocks {
        match block.data {
            BlockData::Dirt => spawn_entity(&mut commands, BlockBundle::new(block.position)),
            BlockData::Enemy {} => spawn_entity(
                &mut commands,
                EnemyBundle::new(
                    block.position,
                    game_assets
                        .image_handles
                        .get(&ImageHandleId::Enemy)
                        .expect("Enemy image assets loaded"),
                ),
            ),
            BlockData::Coin => spawn_entity(
                &mut commands,
                CoinBundle::new(block.position, &asset_server, &mut texture_atlas_layouts),
            ),
        };
    }

    next_state.set(LevelState::Loaded);
}

/// A helper function to spawn an entity with the `LevelEntity` component.
/// This is used to keep track of entities that are part of the level, so
/// they can be easily despawned when the level is cleaned up.
pub fn spawn_entity(commands: &mut Commands, bundle: impl Bundle) {
    commands.spawn(bundle).insert(LevelEntity);
}

pub fn despawn_entities(mut commands: Commands, query: Query<Entity, With<LevelEntity>>) {
    for entity in &query {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}

/// After the game ends, change to state back to waiting for the level to start,
/// so that when the game state changes back to `InGame`, the level will respawn.
fn reprime_level_state(mut next_state: ResMut<NextState<LevelState>>) {
    next_state.set(LevelState::WaitingForLevelStart)
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum LevelState {
    #[default]
    LoadingAssets,
    ConstructingLevel,
    SpawningBlocks,
    WaitingForLevelStart,
    Loaded,
}

#[derive(/*Component,*/ Resource, Default)]
struct GameAsset {
    pub image_handles: HashMap<ImageHandleId, ImageHandles>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum ImageHandleId {
    Enemy,
    Coin,
    BackgroundSwampHills0,
    BackgroundSwampHills1,
    BackgroundSwampHills2,
    BackgroundSwampIsland0,
    BackgroundSwampIsland1,
    BackgroundSwampIsland2,
    BackgroundSwampKelp0,
    BackgroundSwampKelp1,
    BackgroundSwampPond,
}

pub struct ImageHandles {
    pub texture: Handle<Image>,
}

#[derive(Debug, Resource)]
struct LevelHandle(Handle<LevelAsset>);

#[derive(Clone, Debug, Resource)]
struct Level {
    name: String,
    biome: Biome,
    blocks: Vec<Block>,
}

impl From<LevelAsset> for Level {
    fn from(level_asset: LevelAsset) -> Self {
        let blocks = level_asset
            .blocks
            .into_iter()
            .map(|block| Block {
                position: block.position * SIZE,
                ..block
            })
            .collect();

        Level {
            name: level_asset.name,
            biome: level_asset.biome,
            blocks,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, Asset, bevy::reflect::TypePath)]
struct LevelAsset {
    name: String,
    biome: Biome,
    blocks: Vec<Block>,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct Block {
    data: BlockData,
    position: Vec2,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "type")]
enum BlockData {
    Dirt,
    Enemy {},
    Coin,
}

#[derive(Component)]
pub struct LevelEntity;
