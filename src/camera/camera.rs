use std::f32::consts::{FRAC_PI_2, TAU};
use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::Ship;

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct CameraRotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for CameraRotation {
    fn default() -> Self {
        CameraRotation {
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

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
    CameraRotation::default(),
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
    mut target_query: Query<
        (&mut Transform, &mut AngularVelocity, &mut CameraRotation),
        (With<CameraTarget>, Without<Ship>),
    >,
    mut camera_query: Query<(&OrbitCamera, &mut Transform), (Without<CameraTarget>, Without<Ship>)>,
    ship_query: Query<&Transform, (With<Ship>, Without<CameraTarget>)>,
) {
    let dt = time.delta_secs();
    // Only process mouse motion if a button is pressed
    let total_motion: Vec2 = if mouse.any_pressed([MouseButton::Left, MouseButton::Right, MouseButton::Middle]) {
        evr_motion.read().map(|e| e.delta).sum()
    } else {
        Vec2::ZERO
    };

    for (orbit_camera, mut camera_transform) in camera_query.iter_mut() {
        if let Ok((mut target_transform, mut angular_velocity, mut camera_rotation)) =
            target_query.single_mut()
        {
            let mut update_transform = false;
            let rotation_threshold = 0.01; // Minimum rotation change (radians) to trigger transform update
            let velocity_threshold = 0.05; // Higher threshold to zero out velocities faster

            if orbit_camera.orbit_key.map_or(true, |key| mouse.pressed(key)) && total_motion != Vec2::ZERO {
                // Calculate target angular velocity from mouse input
                let target_velocity = total_motion * orbit_camera.orbit_sensitivity;
                let accel_factor = (orbit_camera.acceleration * dt).min(1.0);
                angular_velocity.yaw = angular_velocity.yaw.lerp(-target_velocity.x, accel_factor);
                angular_velocity.pitch = angular_velocity.pitch.lerp(-target_velocity.y, accel_factor);
                update_transform = true;
            } else if angular_velocity.yaw.abs() > velocity_threshold || angular_velocity.pitch.abs() > velocity_threshold {
                // Apply decay only if velocity is significant
                let decay_factor = (-orbit_camera.deacceleration * dt).exp();
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
                camera_rotation.pitch = camera_rotation.pitch.clamp(-FRAC_PI_2 * 0.99, FRAC_PI_2 * 0.99);
                camera_rotation.yaw = camera_rotation.yaw % TAU;

                // Only update transforms if rotation change is significant
                if (camera_rotation.yaw - prev_yaw).abs() > rotation_threshold
                    || (camera_rotation.pitch - prev_pitch).abs() > rotation_threshold
                {
                    // Update target rotation
                    target_transform.rotation =
                        Quat::from_euler(EulerRot::YXZ, camera_rotation.yaw, camera_rotation.pitch, 0.0);

                    // Update camera position and orientation
                    let local_pos = Vec3::new(0.0, 0.0, orbit_camera.radius);
                    camera_transform.translation =
                        target_transform.translation + target_transform.rotation * local_pos;
                    *camera_transform = camera_transform.looking_at(target_transform.translation, Vec3::Y);
                }
            }

            // Follow the ship
            if let Ok(ship_transform) = ship_query.single() {
                if target_transform.translation != ship_transform.translation {
                    target_transform.translation = ship_transform.translation;
                    // Update camera position to follow ship
                    let local_pos = Vec3::new(0.0, 0.0, orbit_camera.radius);
                    camera_transform.translation =
                        target_transform.translation + target_transform.rotation * local_pos;
                    *camera_transform = camera_transform.looking_at(target_transform.translation, Vec3::Y);
                }
            }
        }
    }
}
