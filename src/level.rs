use bevy::prelude::*;

use block::BlockBundle;

mod block;

pub struct Level {
    pub id: u32,
    pub blocks: Vec<BlockBundle>,
}

impl Level {
    pub fn new(id: u32) -> Self {
        debug!("Creating level (id: {})", id);

        let num_blocks = match id {
            0 => 100,
            id => panic!("Unknown level ID: {}", id),
        };

        trace!("Creating {} blocks for level", num_blocks);

        let mut blocks = Vec::with_capacity(num_blocks);
        for i in 0..num_blocks {
            let x = i as f32 * block::SIZE;
            let y = if i % 20 == 0 { block::SIZE } else { 0. } + block::SIZE / 2.0;

            let translation = Vec3::new(x, y, 0.0);
            blocks.push(BlockBundle::new(translation));
        }

        Self { id, blocks }
    }
}

impl Resource for Level {}

pub fn spawn_blocks(mut commands: Commands, level: Res<Level>) {
    debug!("Spawning blocks");
    for block in level.blocks.iter() {
        commands.spawn(block.clone());
    }
}
