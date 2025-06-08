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
