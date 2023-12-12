use bevy::prelude::*;

use self::block::BlockBundle;

mod block;

const SIZE: f32 = 64.0;

#[derive(Default)]
pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blocks);
    }
}

fn spawn_blocks(mut commands: Commands) {
    let block = BlockBundle::new(Vec2::new(0.0, 0.0));
    commands.spawn(block);

    for i in 0..100 {
        let x = i as f32 * SIZE;
        let y = (if i % 8 < 3 { 0 } else { (i - 3) % 5 }) as f32 * SIZE + SIZE;
        let block = BlockBundle::new(Vec2::new(x, y));
        commands.spawn(block);
    }
}
