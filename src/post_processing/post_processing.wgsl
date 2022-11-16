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
    let aberration_output = vec3<f32>(
        textureSample(texture, samp, uv.xy + vec2<f32>(offset, -offset)).r,
        textureSample(texture, samp, uv.xy + vec2<f32>(-offset, 0.0)).g,
        textureSample(texture, samp, uv.xy + vec2<f32>(0.0, offset)).b,
    );

    // vignette
    var dither = vec3<f32>(dot(vec2<f32>(171.0, 231.0), uv.xy * 1000.0)).xxx;
    dither = fract(dither.rgb / vec3<f32>(103.0, 71.0, 97.0)) / 100.0;
    let f_dither = dither.x + dither.y + dither.z;
    let centricity = min((uv.x * (1.0 - uv.x) * uv.y * (1.0 - uv.y) * 10.0), 1.0);
    let intensity = pow(centricity, 0.15);
    let vignette_output = aberration_output * (intensity + f_dither);

    return vec4<f32>(vignette_output, 1.0);
}
