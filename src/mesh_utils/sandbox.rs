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
    {
      let radius = 3.0;
      let num_points = 64; // Points around the circle
      (0..=num_points) // Note the `=` to include the closing point
        .map(|i| {
          let angle = (i as f32) * 2.0 * std::f32::consts::PI / (num_points as f32);
          let x = radius * angle.cos();
          let z = radius * angle.sin();
          Transform::from_xyz(x, 0.0, z)
        })
        .collect::<Vec<_>>()
    },
  );
}
