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
}

use constants::{
  SizingMode,
};
use crate::resources::theme::Theme;
use crate::ui::button::{button, button_system};
use crate::ui::layout::grid;

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
  commands.spawn(grid(&assets, &theme, SizingMode::Fill, 2, 3, 4.0))
    .with_children(|parent| {
      parent.spawn((
        Node {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          ..default()
        },
        BackgroundColor(theme.red.five.into()),
      ));

      parent.spawn((
        Node {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          ..default()
        },
        BackgroundColor(theme.green.five.into()),
      ));

      parent.spawn((
        Node {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          ..default()
        },
        BackgroundColor(theme.blue.five.into()),
      ));

      parent.spawn((
        Node {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          ..default()
        },
        BackgroundColor(theme.red.two.into()),
      ));
    });
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
