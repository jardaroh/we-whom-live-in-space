use std::{f32::consts::{FRAC_PI_2, TAU}, ops::Range};
use bevy::{core_pipeline::{
  bloom::{Bloom, BloomPrefilter},
  tonemapping::Tonemapping,
}, input::mouse::{AccumulatedMouseMotion, MouseWheel}, prelude::*, window::PrimaryWindow};

use crate::Ship;

#[derive(Component)]
pub struct AngularVelocity {
  pub yaw: f32,
  pub pitch: f32,
  pub zoom: f32,
}

impl Default for AngularVelocity {
  fn default() -> Self {
    Self {
      yaw: 0.0,
      pitch: 0.0,
      zoom: 0.0,
    }
  }
}

#[derive(Component)]
pub struct CameraState {
  pub orbit_distance: f32,
  pub pitch: f32,
  pub yaw: f32,
}

impl Default for CameraState {
  fn default() -> Self {
    Self {
      orbit_distance: 20.0,
      yaw: 0.0,
      pitch: 0.0,
    }
  }
}

#[derive(Debug, Resource)]
pub struct CameraSettings {
  pub pitch_speed: f32,
  pub yaw_speed: f32,
  pub pitch_range: Range<f32>,
  pub zoom_speed: f32,
  pub zoom_range: Range<f32>,
  pub acceleration: f32,
  pub deacceleration: f32,
}

impl Default for CameraSettings {
  fn default() -> Self {
    let pitch_limit = FRAC_PI_2 - 0.05; // Avoid gimbal lock

    Self {
      pitch_speed: 0.5,
      yaw_speed: 0.5,
      pitch_range: -pitch_limit..pitch_limit,
      acceleration: 15.0,
      deacceleration: 5.0,
      zoom_speed: 15.0,
      zoom_range: 5.0..50.0,
    }
  }
}

pub fn camera_setup(
  mut commands: Commands,
) {
  commands.spawn((
    Name::new("Camera"),
    Camera3d::default(),
    // Projection::from(PerspectiveProjection {
    //   fov: 60.0_f32.to_radians(),
    //   ..default()
    // }),
    Camera {
      hdr: true,
      clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 1.0)),
      ..default()
    },

    AngularVelocity::default(),
    CameraState::default(),
    Transform::from_xyz(0.0, 0.0, 20.0)
      .looking_at(Vec3::ZERO, Vec3::Y),

    Bloom {
      intensity: 0.9,
      low_frequency_boost: 0.5,
      low_frequency_boost_curvature: 0.95,
      high_pass_frequency: 0.2,
      prefilter: BloomPrefilter {
        threshold: 0.1,
        threshold_softness: 0.8,
      },
      composite_mode: bevy::core_pipeline::bloom::BloomCompositeMode::Additive,
      ..default()
    },
    Tonemapping::TonyMcMapface,
  ));
}

pub fn orbit_system(
    mut camera_query: Query<
        (&mut Transform, &mut AngularVelocity, &mut CameraState),
        With<Camera3d>,
    >,
    camera_settings: Res<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    time: Res<Time>,
    ship_query: Query<&Transform, (With<Ship>, Without<Camera3d>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let dt = time.delta_secs();
    let velocity_threshold = 0.05; // Stop updates when velocities are below this
    let rotation_threshold = 0.0005; // Skip transform updates for tiny rotation changes
    let zoom_threshold = 0.01; // Threshold for zoom changes

    let window_focused = window_query.single().map(|w| w.focused).unwrap_or(true);
    let target = ship_query.single().map_or(Vec3::ZERO, |t| t.translation);

    for (mut transform, mut angular_velocity, mut camera_state) in camera_query.iter_mut() {
        let mut update_transform = false;

        // Handle orbiting (yaw/pitch)
        if window_focused
            && mouse_buttons.pressed(MouseButton::Left)
            && mouse_motion.delta != Vec2::ZERO
        {
            let delta = mouse_motion.delta;
            let target_yaw_velocity = -delta.x * camera_settings.yaw_speed;
            let target_pitch_velocity = -delta.y * camera_settings.pitch_speed;
            let accel_factor = (camera_settings.acceleration * dt).min(1.0); // Relaxed cap
            angular_velocity.yaw = angular_velocity.yaw.lerp(target_yaw_velocity, accel_factor);
            angular_velocity.pitch = angular_velocity.pitch.lerp(target_pitch_velocity, accel_factor);
            update_transform = true;
        } else if angular_velocity.yaw.abs() > velocity_threshold
            || angular_velocity.pitch.abs() > velocity_threshold
        {
            let decay_factor = (1.0 - camera_settings.deacceleration * dt).max(0.0);
            angular_velocity.yaw *= decay_factor;
            angular_velocity.pitch *= decay_factor;
            update_transform = true;

            if angular_velocity.yaw.abs() < velocity_threshold {
                angular_velocity.yaw = 0.0;
            }
            if angular_velocity.pitch.abs() < velocity_threshold {
                angular_velocity.pitch = 0.0;
            }
        } else {
            angular_velocity.yaw = 0.0;
            angular_velocity.pitch = 0.0;
        }

        // Handle zooming
        let mut zoom_input = 0.0;
        if window_focused {
            for ev in mouse_wheel.read() {
                zoom_input += ev.y; // Mouse wheel up (+1) zooms in, down (-1) zooms out
            }
        }
        if zoom_input != 0.0 {
            let target_zoom_velocity = -zoom_input * camera_settings.zoom_speed;
            let accel_factor = (camera_settings.acceleration * dt).min(1.0);
            angular_velocity.zoom = angular_velocity.zoom.lerp(target_zoom_velocity, accel_factor);
            update_transform = true;
        } else if angular_velocity.zoom.abs() > velocity_threshold {
            let decay_factor = (1.0 - camera_settings.deacceleration * dt).max(0.0);
            angular_velocity.zoom *= decay_factor;
            update_transform = true;

            if angular_velocity.zoom.abs() < velocity_threshold {
                angular_velocity.zoom = 0.0;
            }
        } else {
            angular_velocity.zoom = 0.0;
        }

        if update_transform {
            let prev_yaw = camera_state.yaw;
            let prev_pitch = camera_state.pitch;
            let prev_distance = camera_state.orbit_distance;

            // Update rotation and zoom
            camera_state.yaw += angular_velocity.yaw * dt;
            camera_state.pitch += angular_velocity.pitch * dt;
            camera_state.orbit_distance += angular_velocity.zoom * dt;

            // Clamp pitch and wrap yaw
            camera_state.pitch = camera_state
                .pitch
                .clamp(camera_settings.pitch_range.start, camera_settings.pitch_range.end);
            camera_state.yaw = camera_state.yaw % TAU;
            camera_state.orbit_distance = camera_state
                .orbit_distance
                .clamp(camera_settings.zoom_range.start, camera_settings.zoom_range.end);

            // Only update transform if changes are significant
            if (camera_state.yaw - prev_yaw).abs() > rotation_threshold
                || (camera_state.pitch - prev_pitch).abs() > rotation_threshold
                || (camera_state.orbit_distance - prev_distance).abs() > zoom_threshold
            {
                transform.rotation =
                    Quat::from_euler(EulerRot::YXZ, camera_state.yaw, camera_state.pitch, 0.0);
                transform.translation = target - transform.forward() * camera_state.orbit_distance;
            }
        } else if transform.translation != target {
            let forward = transform.forward();
            transform.translation = target - forward * camera_state.orbit_distance;
        }
    }
}
