use bevy::{
    prelude::*,
};

mod resources {
  pub mod theme;
}

mod ui {
    pub mod button;
}

use crate::resources::theme::Theme;

use crate::ui::button::{button, button_system};

// Define game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
  commands.spawn(Camera2d);

  commands.insert_resource(Theme {
    font: assets.load("fonts/FiraSans-Bold.ttf"),
    ..Default::default()
  });

  commands.spawn(button(&assets));
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(Update, button_system)
    .run();
}
