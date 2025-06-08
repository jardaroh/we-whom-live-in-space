#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    forward_io::VertexOutput,
}

// 3D value noise function
fn noise(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f); // Smooth interpolation
    return mix(
        mix(
            mix(hash(i + vec3<f32>(0.0, 0.0, 0.0)), hash(i + vec3<f32>(1.0, 0.0, 0.0)), u.x),
            mix(hash(i + vec3<f32>(0.0, 1.0, 0.0)), hash(i + vec3<f32>(1.0, 1.0, 0.0)), u.x),
            u.y
        ),
        mix(
            mix(hash(i + vec3<f32>(0.0, 0.0, 1.0)), hash(i + vec3<f32>(1.0, 0.0, 1.0)), u.x),
            mix(hash(i + vec3<f32>(0.0, 1.0, 1.0)), hash(i + vec3<f32>(1.0, 1.0, 1.0)), u.x),
            u.y
        ),
        u.z
    );
}

// Simple hash function for noise
fn hash(p: vec3<f32>) -> f32 {
    return fract(sin(dot(p, vec3<f32>(127.1, 311.7, 74.7))) * 43758.5453123);
}

// Star field generation using spherical coordinates
fn stars(pos: vec3<f32>, density: f32) -> f32 {
    let scaled_pos = pos * 100.0; // Adjust frequency for star density
    let val = noise(scaled_pos);
    // Create sharp, bright stars with a high threshold
    let star = smoothstep(0.98, 1.0, val) * 1.0;
    return star * density;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Use view direction instead of world_position for stable stars
    let view_dir = normalize(in.world_position.xyz - view.world_position);
    
    // Background color (deep space blue-black)
    var color = vec3<f32>(0.008, 0.008, 0.012);

    // Add stars based on view direction
    let star_field = stars(view_dir, 50.0);
    color = color + vec3<f32>(star_field); // Bright white stars

    return vec4<f32>(color, 1.0);
}