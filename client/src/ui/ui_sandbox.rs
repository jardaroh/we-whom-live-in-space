use bevy::prelude::*;

use super::ui_window:: {
  WindowManager,
  create_window,
};

pub fn ui_sandbox(
  mut commands: Commands,
  mut window_manager: ResMut<WindowManager>,
) {
  create_window(
    &mut commands,
    &mut window_manager,
    "Test Window",
    Vec2::new(400.0, 300.0),
    Vec2::new(100.0, 100.0),
  );

  create_window(
    &mut commands,
    &mut window_manager,
    "Another Window",
    Vec2::new(250.0, 150.0),
    Vec2::new(200.0, 200.0),
  );
}
