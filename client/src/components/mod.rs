use bevy::{math::DVec3, prelude::*};

#[derive(Component)]
#[allow(dead_code)]
pub struct Acceleration {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Default for Acceleration {
  fn default() -> Self {
    Acceleration {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }
}

#[derive(Component)]
#[allow(dead_code)]
pub struct Ship;

#[derive(Component)]
#[allow(dead_code)]
pub struct PlayerControlled;

#[derive(Component)]
#[allow(dead_code)]
pub struct Mass(pub f64);

#[derive(Component)]
#[allow(dead_code)]
pub struct MaxThrust(pub DVec3);

/// Component to handle smooth interpolation between server rotation updates
#[derive(Component, Debug)]
#[allow(dead_code)]
pub struct RotationInterpolation {
    /// The rotation at the start of the current interpolation period
    pub start_rotation: Quat,
    /// The target rotation we're interpolating towards
    pub target_rotation: Quat,
    /// When this interpolation period started
    pub start_time: f64,
    /// How long this interpolation should take (server update interval)
    pub duration: f64,
}

impl RotationInterpolation {
    /// Create a new interpolation from current to target rotation
    #[allow(dead_code)]
    pub fn new(current: Quat, target: Quat, start_time: f64, duration: f64) -> Self {
        Self {
            start_rotation: current,
            target_rotation: target,
            start_time,
            duration,
        }
    }

    /// Get the interpolated rotation at the given time
    #[allow(dead_code)]
    pub fn get_rotation_at_time(&self, current_time: f64) -> Quat {
        if self.duration <= 0.0 {
            return self.target_rotation;
        }

        let elapsed = current_time - self.start_time;
        let t = (elapsed / self.duration).clamp(0.0, 1.0);
        
        // Use spherical linear interpolation for smooth rotation
        self.start_rotation.slerp(self.target_rotation, t as f32)
    }

    /// Check if this interpolation is complete
    #[allow(dead_code)]
    pub fn is_complete(&self, current_time: f64) -> bool {
        current_time >= self.start_time + self.duration
    }
}
