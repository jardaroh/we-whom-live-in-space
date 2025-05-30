use bevy::{
  prelude::*,
  input_focus::{
    tab_navigation::{TabNavigationPlugin},
    InputDispatchPlugin
  },
};
use super::ui_theme::Theme;
use super::ui_camera::setup_ui_camera;
use super::ui_button::button_system;
use super::ui_checkbox::checkbox_system;
use super::ui_input::text_input_system;
use super::ui_focus::{
  focus_system,
};


pub fn ui_plugin(app: &mut App) {
  app
    .add_plugins((
      InputDispatchPlugin,
      TabNavigationPlugin,
    ))
    .init_resource::<Theme>()
    .add_systems(Startup, setup_ui_camera)
    .add_systems(Update, (
      button_system,
      checkbox_system,
      text_input_system,
      focus_system,
    ));
}
