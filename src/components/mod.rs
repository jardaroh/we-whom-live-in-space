use avian3d::parry::na::coordinates::X;
use bevy::{math::DVec3, prelude::*};

#[derive(Component)]
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
pub struct Ship;

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct Mass(pub f64);

#[derive(Component)]
pub struct MaxThrust(pub DVec3);
