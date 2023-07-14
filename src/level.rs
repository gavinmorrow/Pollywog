use bevy::prelude::*;

use self::block::BlockBundle;

mod block;

pub struct Level<const N: usize> {
    pub id: u32,
    pub blocks: [BlockBundle; N],
}

impl<const N: usize> Level<N> {
    pub fn new(id: u32) -> Self {
        let blocks = (0..N)
            // Map each index to a `BlockBundle` with a translation of `i * 64.0` on the x axis.
            // This will create a line of blocks, each their width apart.
            .map(|i| BlockBundle::new(Vec3::new(i as f32 * 64.0, 0.0, 0.0)))
            .collect::<Vec<_>>()
            .try_into()
            // Unwrap is safe because we know that the length of the vector is equal to N
            // because we created it from a range of N elements (the `(0..N)` line).
            .unwrap();

        Self { id, blocks }
    }
}

impl<const N: usize> Resource for Level<N> {}

pub fn spawn_blocks<const N: usize>(mut commands: Commands, level: Res<Level<N>>) {
    debug!("Spawning blocks");
    for block in level.blocks.iter() {
        commands.spawn(block.clone());
    }
}
