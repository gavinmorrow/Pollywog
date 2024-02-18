use crate::bundles::coin::Coin;
use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;

pub struct CoinPlugin;
impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (print_coins, coin_collisions));
    }
}

#[derive(Component, Debug, Default)]
pub struct CollectCoin {
    number_coins: u32,
}

fn print_coins(collect_coin_entities: Query<(Entity, &CollectCoin)>) {
    for (entity, collect_coin) in &collect_coin_entities {
        println!("{:?}", collect_coin.number_coins);
    }
}

fn coin_collisions(
    mut collisions: Query<(&KinematicCharacterControllerOutput, &mut CollectCoin)>,
    coins: Query<Entity, With<Coin>>,
    mut commands: Commands,
) {
    for (collisions, mut collect_coin) in &mut collisions {
        for collision in &collisions.collisions {
            if coins.contains(collision.entity) {
                collect_coin.number_coins += 1;
                commands.entity(collision.entity).despawn_recursive();
            }
        }
    }
}
