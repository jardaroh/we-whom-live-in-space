use bevy::{
  asset::RenderAssetUsages, pbr:: {
    ExtendedMaterial, OpaqueRendererMethod
  }, prelude::*, reflect::TypePath, render::{
    mesh::{Indices, PrimitiveTopology},
    render_resource::{AsBindGroup, ShaderRef},
  },
};

use super::LineMaterial;

pub fn from_transforms(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ExtendedMaterial<StandardMaterial, LineMaterial>>>,
  points: Vec<Transform>,
) {
  let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all());
  mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points.iter().map(|t| t.translation).collect::<Vec<_>>());

  commands.spawn((
    Mesh3d(meshes.add(mesh)),
    MeshMaterial3d(materials.add(ExtendedMaterial {
      base: StandardMaterial {
        emissive: LinearRgba::rgb(1.2, 1.2, 1.4),
        ..default()
      },
      extension: LineMaterial {
        stroke_width: 1, // Set your desired default value here
      },
    })),
    Transform::from_xyz(0.0, 0.0, 0.0),
  ));
}
