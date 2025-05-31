use std::{f32::consts::{FRAC_PI_2, TAU}, ops::Range};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, render::camera};

use crate::Ship;

#[derive(Component)]
pub struct AngularVelocity {
  pub yaw: f32,
  pub pitch: f32,
}

impl Default for AngularVelocity {
  fn default() -> Self {
    Self {
      yaw: 0.0,
      pitch: 0.0,
    }
  }
}

#[derive(Component)]
pub struct CameraRotation {
  pub yaw: f32,
  pub pitch: f32,
}

impl Default for CameraRotation {
  fn default() -> Self {
    Self {
      yaw: 0.0,
      pitch: 0.0,
    }
  }
}

#[derive(Debug, Resource)]
pub struct CameraSettings {
  pub orbit_distance: f32,
  pub pitch_speed: f32,
  pub yaw_speed: f32,
  pub pitch_range: Range<f32>,
  acceleration: f32,
  deacceleration: f32,
}

impl Default for CameraSettings {
  fn default() -> Self {
    let pitch_limit = FRAC_PI_2 - 0.05; // Avoid gimbal lock

    Self {
      orbit_distance: 20.0,
      pitch_speed: 0.5,
      yaw_speed: 0.5,
      pitch_range: -pitch_limit..pitch_limit,
      acceleration: 15.0,
      deacceleration: 5.0,
    }
  }
}

pub fn camera_setup(
  mut commands: Commands,
) {
  commands.spawn((
    Name::new("Camera"),
    Camera3d::default(),
      Projection::from(PerspectiveProjection {
      fov: 60.0_f32.to_radians(),
      ..default()
    }),

    AngularVelocity::default(),
    CameraRotation::default(),
    Transform::from_xyz(0.0, 0.0, 20.0)
      .looking_at(Vec3::ZERO, Vec3::Y),
  ));
}

pub fn orbit_system(
    mut camera_query: Query<(&mut Transform, &mut AngularVelocity, &mut CameraRotation), With<Camera3d>>,
    camera_settings: Res<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    ship_query: Query<&Transform, (With<Ship>, Without<Camera3d>)>,
) {
    let dt = time.delta_secs();
    let velocity_threshold = 0.05; // Stop updates when velocities are below this
    let rotation_threshold = 0.0005; // Skip transform updates for tiny rotation changes

    let target = ship_query.single().map_or(Vec3::ZERO, |t| t.translation);

    for (mut transform, mut angular_velocity, mut camera_rotation) in camera_query.iter_mut() {
        let mut update_transform = false;

        if mouse_buttons.pressed(MouseButton::Left) && mouse_motion.delta != Vec2::ZERO {
            // Calculate target angular velocity from mouse input
            let delta = mouse_motion.delta;
            let target_yaw_velocity = -delta.x * camera_settings.yaw_speed;
            let target_pitch_velocity = -delta.y * camera_settings.pitch_speed;

            // Accelerate towards target velocity
            let accel_factor = (camera_settings.acceleration * dt).min(0.01);
            angular_velocity.yaw = angular_velocity.yaw.lerp(target_yaw_velocity, accel_factor);
            angular_velocity.pitch = angular_velocity.pitch.lerp(target_pitch_velocity, accel_factor);
            update_transform = true;
        } else if angular_velocity.yaw.abs() > velocity_threshold || angular_velocity.pitch.abs() > velocity_threshold {
            // Apply decay when not orbiting
            let decay_factor = (-camera_settings.deacceleration * dt).exp();
            angular_velocity.yaw *= decay_factor;
            angular_velocity.pitch *= decay_factor;
            update_transform = true;

            // Zero out small velocities
            if angular_velocity.yaw.abs() < velocity_threshold {
                angular_velocity.yaw = 0.0;
            }
            if angular_velocity.pitch.abs() < velocity_threshold {
                angular_velocity.pitch = 0.0;
            }
        }

        if update_transform {
            // Store previous rotation for threshold check
            let prev_yaw = camera_rotation.yaw;
            let prev_pitch = camera_rotation.pitch;

            // Update yaw and pitch
            camera_rotation.yaw += angular_velocity.yaw * dt;
            camera_rotation.pitch += angular_velocity.pitch * dt;

            // Clamp pitch and wrap yaw
            camera_rotation.pitch = camera_rotation
                .pitch
                .clamp(camera_settings.pitch_range.start, camera_settings.pitch_range.end);
            camera_rotation.yaw = camera_rotation.yaw % TAU;

            // Only update transform if rotation change is significant
            if (camera_rotation.yaw - prev_yaw).abs() > rotation_threshold
                || (camera_rotation.pitch - prev_pitch).abs() > rotation_threshold
            {
                // Update camera rotation
                transform.rotation =
                    Quat::from_euler(EulerRot::YXZ, camera_rotation.yaw, camera_rotation.pitch, 0.0);

                // Update camera position
                transform.translation = target - transform.forward() * camera_settings.orbit_distance;
            }
        } else if transform.translation != target {
          transform.translation = target - transform.forward() * camera_settings.orbit_distance;
        }
    }
}
