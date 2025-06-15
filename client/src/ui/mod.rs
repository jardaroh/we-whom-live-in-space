pub mod ui_theme;
pub mod ui_button;
pub mod ui_checkbox;
pub mod ui_input;
pub mod ui_focus;
pub mod ui_window;
pub mod ui_sandbox;

use bevy::{
  prelude::*,
  input_focus::{
    tab_navigation::TabNavigationPlugin,
    InputDispatchPlugin,
    InputFocus,
  },
};
use ui_theme::Theme;
use ui_button::button_system;
use ui_checkbox::checkbox_system;
use ui_input::{
  text_input_system,
  text_input_click_system,
};
use ui_focus::{
  focus_system,
};
use ui_window::WindowPlugin;
use ui_sandbox::ui_sandbox;


pub fn ui_plugin(app: &mut App) {
  app
    .add_plugins((
      InputDispatchPlugin,
      TabNavigationPlugin,
      WindowPlugin,
    ))
    .init_resource::<Theme>()
    .init_resource::<InputFocus>()
    .add_systems(Startup, ui_sandbox)
    .add_systems(Update, (
      button_system,
      checkbox_system,
      text_input_system,
      text_input_click_system,
      focus_system,
    ));
}
