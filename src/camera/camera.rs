use std::f32::consts::{FRAC_PI_2, TAU};
use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::Ship;

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct OrbitCamera {
  pub radius: f32,
  pub orbit_sensitivity: f32,
  pub orbit_key: Option<MouseButton>,
  pub acceleration: f32,
  pub deacceleration: f32,
}

impl Default for OrbitCamera {
  fn default() -> Self {
    OrbitCamera {
      radius: 10.0,
      orbit_sensitivity: 25.0_f32.to_radians(),
      orbit_key: Some(MouseButton::Left),
      acceleration: 10.0,
      deacceleration: 2.0,
    }
  }
}

#[derive(Component)]
pub struct AngularVelocity {
  pub yaw: f32,
  pub pitch: f32,
}

impl Default for AngularVelocity {
  fn default() -> Self {
    AngularVelocity {
      yaw: 0.0,
      pitch: 0.0,
    }
  }
}

pub fn camera_setup(
  mut commands: Commands,
) {
  commands.spawn((
    CameraTarget,
    AngularVelocity::default(),
    Transform::from_xyz(0.0, 0.0, 0.0),
  ))
  .with_children(|parent| {
    parent.spawn((
      OrbitCamera {
        radius: 10.0,
        orbit_sensitivity: 25.0_f32.to_radians(),
        orbit_key: Some(MouseButton::Left),
        ..default()
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
  mut target_query: Query<(&mut Transform, &mut AngularVelocity), (With<CameraTarget>, Without<Ship>)>,
  mut camera_query: Query<(&OrbitCamera, &mut Transform), (Without<CameraTarget>, Without<Ship>)>,
  ship_query: Query<&Transform, (With<Ship>, Without<CameraTarget>)>,
) {
  let total_motion: Vec2 = evr_motion.read().map(|e| e.delta).sum();
  let dt = time.delta_secs();

  for (orbit_camera, mut camera_transform) in camera_query.iter_mut() {
    if let Ok((mut target_transform, mut angular_velocity)) = target_query.single_mut() {
      if orbit_camera.orbit_key.map_or(true, |key| mouse.pressed(key)) {
        let orbit = total_motion * orbit_camera.orbit_sensitivity;
        let accel_factor = (orbit_camera.acceleration * dt).min(1.0).powi(2);
        angular_velocity.yaw = angular_velocity.yaw.lerp(-orbit.x, accel_factor);
        angular_velocity.pitch = angular_velocity.pitch.lerp(-orbit.y, accel_factor);
      } else {
        let decay_factor = (-orbit_camera.deacceleration * dt).exp();
        angular_velocity.yaw *= decay_factor;
        angular_velocity.pitch *= decay_factor;

        if angular_velocity.yaw.abs() < 0.001 {
          angular_velocity.yaw = 0.0;
        }
        if angular_velocity.pitch.abs() < 0.001 {
          angular_velocity.pitch = 0.0;
        }
      }

      let euler = target_transform.rotation.to_euler(EulerRot::YXZ);
      let mut yaw = euler.0 + angular_velocity.yaw * dt;
      let mut pitch = euler.1 + angular_velocity.pitch * dt;

      pitch = pitch.clamp(-FRAC_PI_2 * 0.5, FRAC_PI_2 * 0.5);
      yaw = yaw % TAU;

      // Clamp pitch and wrap yaw
      pitch = pitch.clamp(-FRAC_PI_2 * 0.5, FRAC_PI_2 * 0.5);
      yaw = yaw % TAU;

      target_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

      // Update camera position and orientation
      let local_pos = Vec3::new(0.0, 0.0, orbit_camera.radius);
      camera_transform.translation = target_transform.translation + target_transform.rotation * local_pos;
      *camera_transform = camera_transform.looking_at(target_transform.translation, Vec3::Y);

      if let Ok(ship_transform) = ship_query.single() {
        target_transform.translation = ship_transform.translation;
      }
    }
  }
}
