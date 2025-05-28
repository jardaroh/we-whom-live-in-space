use bevy::{
  prelude::*,
};

mod constants;

mod resources {
  pub mod theme;
}

mod ui {
  pub mod button;
  pub mod layout;
  pub mod main_menu;
}

use constants::{
  SizingMode,
};
use crate::resources::theme::Theme;
use crate::ui::button::{button, button_system};
use crate::ui::layout::grid;
use crate::ui::main_menu::main_menu;

// Define game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
  #[default]
  MainMenu,
  InGame,
}

fn setup_theme(mut commands: Commands, assets: Res<AssetServer>) {
  commands.insert_resource(Theme {
    font: assets.load("fonts/FiraSans-Bold.ttf"),
    ..Default::default()
  });
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
  commands.spawn(Camera2d);
}

fn setup_ui(mut commands: Commands, assets: Res<AssetServer>, theme: Res<Theme>) {
  commands.spawn(main_menu(
    &assets,
    &theme,
  ));
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .init_state::<GameState>()
    .add_systems(PreStartup, setup_theme)
    .add_systems(Startup, setup)
    .add_systems(Startup, setup_ui)
    .add_systems(Update, button_system)
    .run();
}
