use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;

use super::curve;
use super::LineMaterial;

#[derive(Component)]
pub struct PulsingCircle {
  pub base_radius: f32,
  pub num_points: u32,
  pub pulse_amplitude: f32,
  pub pulse_frequency: f32,
  pub mesh_handle: Handle<Mesh>,
}

pub fn setup_mesh_utils_sandbox(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, LineMaterial>>>,
) {
  let base_radius = 4.0;
  let num_points = 6; // Points around the circle
  let pulse_frequency = 1.0; // Pulses per second
  let pulse_amplitude = 0.5; // Amplitude of the pulse
  
  let mut points = (0..=num_points) // Note the `=` to include the closing point
    .map(|i| {
      let angle = (i as f32) * 2.0 * std::f32::consts::PI / (num_points as f32);
      let x = base_radius * angle.cos();
      let z = base_radius * angle.sin();
      Transform::from_xyz(x, 0.0, z)
    })
    .collect::<Vec<_>>();

  let bundle = curve::from_transforms(
    &mut meshes,
    &mut materials,
    points,
  );
  let mesh_handle = bundle.mesh.0.clone();

  commands.spawn((
    bundle,
    PulsingCircle {
      base_radius,
      num_points,
      pulse_amplitude,
      pulse_frequency,
      mesh_handle: mesh_handle.clone(),
    },
  ));
}

pub fn update_pulsing_circle(
  time: Res<Time>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut query: Query<&PulsingCircle>,
) {
  for pulsing_circle in query.iter_mut() {
    if let Some(mesh) = meshes.get_mut(&pulsing_circle.mesh_handle) {
      let current_time = time.elapsed_secs();
      let radius = pulsing_circle.base_radius
        + pulsing_circle.pulse_amplitude * (current_time * pulsing_circle.pulse_frequency).sin();
      
      let positions = (0..=pulsing_circle.num_points)
        .map(|i| {
          let angle = (i as f32) * 2.0 * std::f32::consts::PI / (pulsing_circle.num_points as f32);
          Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin())
        })
        .collect::<Vec<_>>();
      mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        positions,
      );
    }
  }
}
