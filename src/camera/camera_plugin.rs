use bevy::{log::tracing_subscriber::field::debug, prelude::*};

use crate::camera::camera::{
  camera_setup,
  orbit_system,
  CameraSettings,
  //debug_camera_system,
};

pub fn camera_plugin(app: &mut App) {
  app
    .init_resource::<CameraSettings>()
    .add_systems(Startup, camera_setup)
    .add_systems(Update, (
      orbit_system,
      //debug_camera_system,
    ));
}
