use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Win,
    Dead,
}
