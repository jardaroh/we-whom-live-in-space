use std::f32::consts::{FRAC_PI_2, TAU};
use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::Ship;

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct OrbitCamera {
  pub radius: f32,
  orbit_sensitivity: f32,
  orbit_key: Option<MouseButton>,
}

impl Default for OrbitCamera {
  fn default() -> Self {
    OrbitCamera {
      radius: 10.0,
      orbit_sensitivity: 1.0_f32.to_radians(),
      orbit_key: Some(MouseButton::Left),
    }
  }
}

pub fn camera_setup(
  mut commands: Commands,
) {
  commands.spawn((
    CameraTarget,
    Transform::from_xyz(0.0, 0.0, 0.0),
  ))
  .with_children(|parent| {
    parent.spawn((
      OrbitCamera {
        radius: 10.0,
        orbit_sensitivity: 1.0_f32.to_radians(),
        orbit_key: Some(MouseButton::Left),
      },
      Camera3d {
        ..default()
      },
      Projection::from(PerspectiveProjection {
        fov: 60.0_f32.to_radians(),
        near: 0.1,
        far: 1000.0,
        ..default()
      }),
      Transform::from_xyz(0.0, 0.0, 10.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
    ));
  });
}

pub fn orbit_camera_system(
  time: Res<Time>,
  mouse: Res<ButtonInput<MouseButton>>,
  mut evr_motion: EventReader<MouseMotion>,
  mut target_query: Query<&mut Transform, (With<CameraTarget>, Without<Ship>)>,
  mut camera_query: Query<(&OrbitCamera, &mut Transform), (Without<CameraTarget>, Without<Ship>)>,
  ship_query: Query<&Transform, (With<Ship>, Without<CameraTarget>)>,
) {
  let total_motion: Vec2 = evr_motion.read().map(|e| e.delta).sum();

  for (orbit_camera, mut camera_transform) in camera_query.iter_mut() {
    if let Ok(mut target_transform) = target_query.single_mut() {
      if orbit_camera.orbit_key.map_or(true, |key| mouse.pressed(key)) {
        let orbit = total_motion * orbit_camera.orbit_sensitivity;
        let mut euler = target_transform.rotation.to_euler(EulerRot::YXZ);
        let mut yaw = euler.0 - orbit.x;
        let mut pitch = euler.1 - orbit.y;

        pitch = pitch.clamp(-FRAC_PI_2 * 0.5, FRAC_PI_2 * 0.5);
        yaw = yaw % TAU;

        let raw_lerp_factor = (50.0 * time.delta_secs()).min(0.1);
        let lerp_factor = EaseFunction::QuadraticInOut.sample_clamped(raw_lerp_factor);

        euler.0 = euler.0.lerp(yaw, lerp_factor);
        euler.1 = euler.1.lerp(pitch, lerp_factor);

        target_transform.rotation = Quat::from_euler(EulerRot::YXZ, euler.0, euler.1, 0.0);

        let local_pos = Vec3::new(0.0, 0.0, orbit_camera.radius);
        camera_transform.translation = target_transform.translation + target_transform.rotation * local_pos;
        *camera_transform = camera_transform.looking_at(target_transform.translation, Vec3::Y);
      }
    }

    if let Ok(ship_transform) = ship_query.single() {
    for mut target_transform in target_query.iter_mut() {
      target_transform.translation = ship_transform.translation;
    }
  }
  }
}
