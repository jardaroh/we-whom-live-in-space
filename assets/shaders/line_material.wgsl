#import bevy_pbr::view_transformations::position_world_to_clip
#import bevy_pbr::mesh_view_bindings::view

struct LineMaterial {
  stroke_width: u32,
}

@group(2) @binding(100)
var<uniform> material: LineMaterial;

struct VertexInput {
  @location(0) position: vec3<f32>,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) color: vec4<f32>,
}

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  
  // Simple transform: treat input position as world position
  out.clip_position = position_world_to_clip(vertex.position);
  
  // Set a base color, modulated by stroke_width for visual effect
  let intensity = f32(material.stroke_width) / 10.0; // Normalize stroke_width for color
  out.color = vec4<f32>(0.2, 0.6, 1.0, 1.0) * clamp(intensity, 0.5, 1.0); // Blue-ish color for space aesthetic
  return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
  return in.color;
}