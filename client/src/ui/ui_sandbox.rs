use bevy::{prelude::*, ui::update};

use super::ui_window:: {
  WindowManager,
  WindowContentArea,
  create_window,
  update_window_content,
};

pub fn ui_sandbox(
  mut commands: Commands,
  mut window_manager: ResMut<WindowManager>,
  content_area_query: Query<(Entity, &WindowContentArea)>,
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

  let window_to_update = create_window(
    &mut commands,
    &mut window_manager,
    "Third Window",
    Vec2::new(350.0, 200.0),
    Vec2::new(300.0, 250.0),
  );

  update_window_content(
    &mut window_manager,
    window_to_update,
    |parent| {
      parent.spawn((
        Text::new("New content"),
        TextFont {
          font_size: 16.0,
          ..default()
        },
        TextColor(Color::srgb(0.5, 0.9, 0.5)),
      ));
    },
  );
}
