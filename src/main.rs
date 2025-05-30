use bevy::{
  prelude::*,
  winit::WinitSettings,
  input_focus::tab_navigation::TabGroup,
};

mod constants;

mod ui;

use ui::ui_theme::Theme;
use ui::ui_plugin::ui_plugin;
use ui::ui_button::button;
use ui::ui_checkbox::checkbox;
use ui::ui_input::text_input;

fn setup_ui_test(mut commands: Commands, theme: Res<Theme>) {
  commands.spawn((
    Node {
      display: Display::Grid,
      grid_template_columns: vec![GridTrack::min_content(); 1],
      grid_template_rows: vec![GridTrack::min_content(); 4],
      row_gap: Val::Px(6.0),
      ..default()
    },
    TabGroup::new(0),
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
      text_input(
        &theme,
        "Type here...",
      ),
    ],
  ));
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(WinitSettings::desktop_app())
    .add_plugins(ui_plugin)
    .add_systems(Startup, setup_ui_test)
    
    .run();
}
