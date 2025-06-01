use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;

use super::curve;
use super::LineMaterial;

pub fn setup_mesh_utils_sandbox(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, LineMaterial>>>,
) {
  // This function is a placeholder for setting up the mesh utils sandbox.
  // You can add any initialization logic here if needed.
  curve::from_transforms(
    &mut commands,
    &mut meshes,
    &mut materials,
    vec![
      Transform::from_xyz(0.0, 0.0, 0.0),
      Transform::from_xyz(1.0, 1.0, 0.0),
      Transform::from_xyz(2.0, 0.0, 0.0),
    ],
  );
}
