use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierContext;

use crate::{
    plugins::in_game::{bundles::coin::Coin, level, player::Player},
    state::GameState,
};

const TEXT_POS: Vec2 = Vec2::new(10.0, 10.0);
const FONT_SIZE: f32 = 64.0;

pub fn coin_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), create_text)
        .add_systems(
            Update,
            (update_coin_score, coin_collisions).run_if(in_state(GameState::InGame)),
        );
}

#[derive(Component, Debug, Default)]
pub struct CoinCollector {
    num_coins: u32,
}

#[derive(Component)]
struct CoinScoreText;

fn create_text(mut commands: Commands) {
    info!("Creating score text.");

    commands
        .spawn(TextBundle {
            style: Style {
                top: Val::Px(TEXT_POS.x),
                left: Val::Px(TEXT_POS.y),
                ..default()
            },
            text: Text::from_section(
                "-",
                TextStyle {
                    font_size: FONT_SIZE,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(CoinScoreText)
        .insert(level::LevelEntity);
}

fn update_coin_score(
    player: Query<&CoinCollector, With<Player>>,
    mut score_text: Query<&mut Text, With<CoinScoreText>>,
) {
    let player = player.single();
    let mut score_text = score_text.single_mut();
    score_text.sections[0].value = player.num_coins.to_string();
}

fn coin_collisions(
    // Is this use of RapierContext correct, or is there a better way to do it?
    rapier_context: Res<RapierContext>,
    mut collectors: Query<(Entity, &mut CoinCollector)>,
    coins: Query<Entity, With<Coin>>,
    mut commands: Commands,
) {
    for (entity, mut collector) in &mut collectors {
        for coin in &coins {
            if rapier_context.intersection_pair(entity, coin) == Some(true) {
                // They are intersecting
                commands.entity(coin).despawn_recursive();
                collector.num_coins += 1;
                debug!(
                    "{:?} collected a coin {:?}! new total: {}",
                    entity, coin, collector.num_coins
                );
            }
        }
    }
}
