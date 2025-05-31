#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

// Simple hash function for noise
fn hash(p: vec3<f32>) -> f32 {
    return fract(sin(dot(p, vec3<f32>(127.1, 311.7, 74.7))) * 43758.5453123);
}

// Star field generation using 3D position
fn stars(pos: vec3<f32>, density: f32) -> f32 {
    let val = hash(pos * 50.0); // Lower frequency for better distribution
    let star = smoothstep(0.996, 1.0, val) * 1.0; // Crisp, bright stars
    return star * density;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = normalize(in.world_position.xyz); // Normalize for spherical mapping

    // Background color (deep space blue-black)
    var color = vec3<f32>(0.008, 0.008, 0.012);

    // Add stars
    let star_field = stars(pos, 1.0);
    color = color + vec3<f32>(star_field); // Bright white stars

    return vec4<f32>(color, 1.0);
}