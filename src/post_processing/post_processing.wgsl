#import bevy_core_pipeline::fullscreen_vertex_shader

struct Params {
    aberration: f32,
};

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var samp: sampler;
@group(0) @binding(2)
var<uniform> params: Params;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(textureDimensions(texture));
    let uv = in.position.xy / resolution;

    // chromatic aberration
    let offset = params.aberration;
    var output = vec4<f32>(
        textureSample(texture, samp, uv + vec2<f32>(offset, -offset)).r,
        textureSample(texture, samp, uv + vec2<f32>(-offset, 0.0)).g,
        textureSample(texture, samp, uv + vec2<f32>(0.0, offset)).b,
        1.0
    );

    return output;
}
