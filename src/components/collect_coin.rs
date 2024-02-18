use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;

use crate::bundles::{coin::Coin, player::Player};

pub struct CoinPlugin;
impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_text)
            .add_systems(Update, (update_coin_score, coin_collisions));
    }
}

#[derive(Component, Debug, Default)]
pub struct CoinCollector {
    number_coins: u32,
}

#[derive(Component)]
struct CoinScoreText;

fn create_text(mut commands: Commands) {
    info!("Creating score text.");

    commands
        .spawn(TextBundle {
            style: Style {
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            text: Text::from_section(
                "0",
                TextStyle {
                    font_size: 64.0,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(CoinScoreText);
}

fn update_coin_score(
    collect_coin_entities: Query<&CoinCollector, With<Player>>,
    mut score_text: Query<&mut Text, With<CoinScoreText>>,
) {
    let collect_coin = collect_coin_entities.single();
    let mut score_text = score_text.single_mut();
    score_text.sections[0].value = collect_coin.number_coins.to_string();
}

fn coin_collisions(
    mut collisions: Query<(&KinematicCharacterControllerOutput, &mut CoinCollector)>,
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
