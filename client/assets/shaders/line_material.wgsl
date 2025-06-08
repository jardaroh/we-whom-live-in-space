#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct LineMaterial {
    stroke_width: u32,
}

@group(2) @binding(100)
var<uniform> extension: LineMaterial;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Generate a standard PBR input
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    
    // Modify the base color for your line effect
    let line_color = vec4<f32>(0.2, 0.6, 1.0, 1.0);
    pbr_input.material.base_color = line_color;

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    
    // Add emissive glow for HDR after lighting calculations
    out.color = out.color + vec4<f32>(1.0, 3.0, 5.0, 0.0) * 2.0;
#endif

    return out;
}