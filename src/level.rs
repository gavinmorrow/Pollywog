use std::collections::HashMap;

use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::bundles::{coin::CoinBundle, enemy::EnemyBundle};

use self::block::BlockBundle;

mod block;

const SIZE: f32 = 64.0;

#[derive(Default)]
pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LevelState>()
            .insert_resource(GameAsset::default())
            .add_plugins(JsonAssetPlugin::<LevelAsset>::new(&["level.json"]))
            .add_systems(
                OnEnter(LevelState::LoadingAssets),
                (load_level_asset, load_image_assets),
            )
            .add_systems(
                Update,
                wait_for_level_load.run_if(state_exists_and_equals(LevelState::LoadingAssets)),
            )
            .add_systems(OnEnter(LevelState::ConstructingLevel), construct_level_res)
            .add_systems(OnEnter(LevelState::SpawningBlocks), spawn_blocks);
    }
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
    next_state.set(LevelState::SpawningBlocks);
}

fn load_image_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAsset>) {
    game_assets.image_handles = std::collections::HashMap::from([
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
}

fn spawn_blocks(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelState>>,
    level: Res<Level>,
    game_assets: Res<GameAsset>,
) {
    info!("Spawning blocks for level: {}", level.name);
    for block in &level.blocks {
        match block.data {
            BlockData::Dirt => {
                let block = BlockBundle::new(block.position);
                commands.spawn(block);
            }
            BlockData::Enemy {} => {
                let enemy = EnemyBundle::new(
                    block.position,
                    game_assets
                        .image_handles
                        .get(&ImageHandleId::Enemy)
                        .expect("Enemy image assets loaded"),
                );
                commands.spawn(enemy);
            }
            BlockData::Coin => {
                let coin = CoinBundle::new(
                    block.position,
                    game_assets
                        .image_handles
                        .get(&ImageHandleId::Coin)
                        .expect("Coin image assets loaded"),
                );
                commands.spawn(coin);
            }
        }
    }
    next_state.set(LevelState::Loaded);
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum LevelState {
    #[default]
    LoadingAssets,
    ConstructingLevel,
    SpawningBlocks,
    Loaded,
}

#[derive(/*Component,*/ Resource, Default)]
struct GameAsset {
    pub image_handles: HashMap<ImageHandleId, ImageHandles>,
}

#[derive(Eq, PartialEq, Hash)]
enum ImageHandleId {
    Enemy,
    Coin,
}

pub struct ImageHandles {
    pub texture: Handle<Image>,
}

#[derive(Debug, Resource)]
struct LevelHandle(Handle<LevelAsset>);

#[derive(Clone, Debug, Resource)]
struct Level {
    name: String,
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
            blocks,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, Asset, bevy::reflect::TypePath)]
struct LevelAsset {
    name: String,
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
