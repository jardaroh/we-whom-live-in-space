#import bevy_pbr::view_transformations::position_world_to_clip
#import bevy_pbr::mesh_view_bindings::view

struct LineMaterial {
  stroke_width: u32,
  color: vec4<f32>,
}

@group(2) @binding(100)
var<uniform> material: LineMaterial;

struct VertexInput {
  @location(0) position: vec3<f32>,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) world_position: vec3<f32>,
}

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.clip_position = position_world_to_clip(vertex.position);
  out.world_position = vertex.position;
  return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
  // Return a solid color, ignoring any lighting
  return vec4<f32>(0.2, 0.6, 1.0, 1.0);
}