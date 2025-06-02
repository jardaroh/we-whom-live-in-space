use bevy::{
  asset::RenderAssetUsages, pbr:: {
    ExtendedMaterial, OpaqueRendererMethod
  }, prelude::*, reflect::TypePath, render::{
    mesh::{Indices, PrimitiveTopology},
    render_resource::{AsBindGroup, ShaderRef},
  },
};

use super::LineMaterial;

#[derive(Bundle, Default)]
pub struct LineMeshBundle {
  pub mesh: Mesh3d,
  pub material: MeshMaterial3d<ExtendedMaterial<StandardMaterial, LineMaterial>>,
  pub transform: Transform,
}

pub fn from_transforms(
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ExtendedMaterial<StandardMaterial, LineMaterial>>>,
  points: Vec<Transform>,
) -> LineMeshBundle {
  let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all());
  mesh.insert_attribute(
    Mesh::ATTRIBUTE_POSITION,
    points.iter().map(|t| t.translation).collect::<Vec<_>>(),
  );

  LineMeshBundle {
    mesh: Mesh3d(meshes.add(mesh)),
    material: MeshMaterial3d(materials.add(ExtendedMaterial {
      base: StandardMaterial {
        emissive: LinearRgba::rgb(1.2, 1.2, 1.4),
        ..default()
      },
      extension: LineMaterial {
        stroke_width: 1, // Set your desired default value here
      },
    })),
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
  }
}
