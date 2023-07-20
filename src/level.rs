use bevy::prelude::*;

use block::{BlockBundle, JumpCollisionBoxBundle};

pub mod block;

#[derive(Default)]
pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::new(0))
            .add_systems(Startup, spawn_blocks);
    }
}

struct Level {
    // this will be used soon
    _id: u32,
    blocks: Vec<BlockBundle>,
    jump_collision_block_boxes: Vec<JumpCollisionBoxBundle>,
}

impl Level {
    fn new(id: u32) -> Self {
        debug!("Creating level (id: {})", id);

        let num_blocks = match id {
            0 => 100,
            id => panic!("Unknown level ID: {}", id),
        };

        trace!("Creating {} blocks for level", num_blocks);

        let mut blocks = Vec::with_capacity(num_blocks);
        let mut jump_collision_block_boxes = Vec::with_capacity(num_blocks);
        for i in 0..num_blocks {
            let x = i as f32 * block::SIZE;
            let y = if i % 20 < 15 {
                0.0
            } else {
                (i % 20 - 15) as f32 * block::SIZE
            };

            let translation = Vec3::new(x, y, 0.0);
            blocks.push(BlockBundle::new(translation));
            jump_collision_block_boxes.push(JumpCollisionBoxBundle::new(translation));
        }

        Self {
            _id: id,
            blocks,
            jump_collision_block_boxes,
        }
    }
}

impl Resource for Level {}

fn spawn_blocks(mut commands: Commands, level: Res<Level>) {
    debug!("Spawning blocks");
    for block in level.blocks.iter() {
        commands.spawn(block.clone());
    }
    for jump_collision_block_box in level.jump_collision_block_boxes.iter() {
        commands.spawn(jump_collision_block_box.clone());
    }
}
