use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;

use crate::{
    bundles::{coin::Coin, player::Player},
    level,
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
    mut collisions: Query<(&KinematicCharacterControllerOutput, &mut CoinCollector)>,
    coins: Query<&Coin>,
    mut commands: Commands,
) {
    for (collisions, mut coin_collector) in &mut collisions {
        for collision in &collisions.collisions {
            if coins.contains(collision.entity) {
                coin_collector.num_coins += 1;
                commands.entity(collision.entity).despawn_recursive();
            }
        }
    }
}
