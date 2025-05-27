use bevy::prelude::*;

pub enum SizingMode {
    Fixed { width: Val, height: Val},
    Fill,
    FitContent,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
  #[default]
  MainMenu,
  InGame,
}
