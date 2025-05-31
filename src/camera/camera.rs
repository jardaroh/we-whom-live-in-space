use std::{f32::consts::FRAC_PI_2, ops::Range};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::Ship;

#[derive(Debug, Resource)]
pub struct CameraSettings {
  pub orbit_distance: f32,
  pub pitch_speed: f32,
  pub yaw_speed: f32,
  pub pitch_range: Range<f32>,
}

impl Default for CameraSettings {
  fn default() -> Self {
    let pitch_limit = FRAC_PI_2 - 0.05; // Avoid gimbal lock

    Self {
      orbit_distance: 20.0,
      pitch_speed: 0.01,
      yaw_speed: 0.01,
      pitch_range: -pitch_limit..pitch_limit,
    }
  }
}

pub fn camera_setup(
  mut commands: Commands,
) {
  commands.spawn((
    Name::new("Camera"),
    Camera3d::default(),
    Transform::from_xyz(0.0, 0.0, 20.0)
      .looking_at(Vec3::ZERO, Vec3::Y),
  ));
}

pub fn orbit_system(
  mut camera: Single<&mut Transform, With<Camera3d>>,
  camera_settings: Res<CameraSettings>,
  mouse_buttons: Res<ButtonInput<MouseButton>>,
  mouse_motion: Res<AccumulatedMouseMotion>,
  time: Res<Time>,
) {
  if mouse_buttons.pressed(MouseButton::Left) {
    let delta = mouse_motion.delta;
    let delta_pitch = delta.y * camera_settings.pitch_speed;
    let delta_yaw = delta.x * camera_settings.yaw_speed;

    let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

    let pitch = (pitch + delta_pitch).clamp(
      camera_settings.pitch_range.start,
      camera_settings.pitch_range.end,
    );
    let yaw = yaw + delta_yaw;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    println!("Camera rotation: {:?}", camera.rotation);
    let target = Vec3::ZERO;
    camera.translation = target - camera.forward() * camera_settings.orbit_distance;
  }
}
