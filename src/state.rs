use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    StartScreen,
    InGame,
    Win,
    Dead,
}
