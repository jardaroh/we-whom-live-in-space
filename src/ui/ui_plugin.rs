use bevy::prelude::*;
use super::ui_theme::Theme;
use super::ui_camera::setup_ui_camera;
use super::ui_button::button_system;
use super::ui_checkbox::checkbox_system;


pub fn ui_plugin(app: &mut App) {
  app
    .init_resource::<Theme>()
    .add_systems(Startup, setup_ui_camera)
    .add_systems(Update, (
      button_system,
      checkbox_system,
    ));
}
