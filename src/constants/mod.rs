use bevy::prelude::*;

pub enum SizingMode {
    Fixed,
    Fill,
    FitContent,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
  #[default]
  MainMenu,
  InGame,
}
