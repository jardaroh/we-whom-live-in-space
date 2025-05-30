use bevy::prelude::*;

pub fn setup_ui_camera(mut commands: Commands) {
  commands.spawn((
    Camera {
      order: 1,
      ..default()
    },
    Camera2d,
    Transform::default(),
    GlobalTransform::default(),
  ));
}
