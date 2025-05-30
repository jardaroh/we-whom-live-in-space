use bevy::{
  prelude::*,
};

mod constants;

mod ui;

use ui::ui_theme::Theme;
use ui::ui_plugin::ui_plugin;
use ui::ui_button::button;
use ui::ui_checkbox::checkbox;

fn setup_ui_test(mut commands: Commands, theme: Res<Theme>) {
  commands.spawn((
    Node {
      display: Display::Grid,
      grid_template_columns: vec![GridTrack::min_content(); 1],
      grid_template_rows: vec![GridTrack::min_content(); 3],
      ..default()
    },
    children![
      button(
        &theme,
        "Click Me",
        (Val::Px(200.0), Val::Px(50.0)),
      ),
      checkbox(
        &theme,
        false,
      ),
      checkbox(
        &theme,
        true,
      ),
    ],
  ));
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(ui_plugin)
    .add_systems(Startup, setup_ui_test)
    
    .run();
}
