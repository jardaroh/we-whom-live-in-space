use bevy::{
  prelude::*,
  render::{
    render_resource::{AsBindGroup, ShaderRef},
    mesh::Mesh,
  },
  reflect::TypePath,
};

const SHADER_ASSET_PATH: &str = "shaders/backdrop.wgsl";

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct BackdropMaterial {}

impl Material for BackdropMaterial {
  fn fragment_shader() -> ShaderRef {
    SHADER_ASSET_PATH.into()
  }

  fn alpha_mode(&self) -> AlphaMode {
    AlphaMode::Opaque
  }
}

pub fn setup_backdrop(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<BackdropMaterial>>,
) {
  let skybox_mesh = meshes.add(Mesh::from(Cuboid::new(1000.0, 1000.0, 1000.0)));

  // commands.spawn((
  //   Mesh3d(skybox_mesh),
  //   MeshMaterial3d(materials.add(BackdropMaterial {})),
  //   Transform::from_scale(Vec3::splat(-1.0)),
  // ));
}

pub fn backdrop_system(
  mut commands: Commands,
  time: Res<Time>,
  material: ResMut<Assets<BackdropMaterial>>,
) {

}
