use bevy::{
  prelude::*,
  pbr::{
    ExtendedMaterial, MaterialExtension,
  },
};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub mod curve;
pub mod sandbox;

use crate::mesh_utils::sandbox::setup_mesh_utils_sandbox;


#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct LineMaterial {
    #[uniform(100)]
    pub stroke_width: u32,
}

impl MaterialExtension for LineMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/line_material.wgsl".into()
  }
}

pub fn mesh_utils_plugin(app: &mut App) {
  app.add_plugins(MaterialPlugin::<
    ExtendedMaterial<StandardMaterial, LineMaterial>,
  >::default())
    .add_systems(Startup, setup_mesh_utils_sandbox);
}
