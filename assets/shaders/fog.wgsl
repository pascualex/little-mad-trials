#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

// helper functions

fn permute_4(x: vec4<f32>) -> vec4<f32> {
    return ((x * 34. + 1.) * x) % vec4<f32>(289.);
}

fn fade_2(t: vec2<f32>) -> vec2<f32> {
    return t * t * t * (t * (t * 6. - 15.) + 10.);
}

fn perlin_noise(P: vec2<f32>) -> f32 {
  var Pi: vec4<f32> = floor(P.xyxy) + vec4<f32>(0., 0., 1., 1.);
  let Pf = fract(P.xyxy) - vec4<f32>(0., 0., 1., 1.);
  Pi = Pi % vec4<f32>(289.); // To avoid truncation effects in permutation
  let ix = Pi.xzxz;
  let iy = Pi.yyww;
  let fx = Pf.xzxz;
  let fy = Pf.yyww;
  let i = permute_4(permute_4(ix) + iy);
  var gx: vec4<f32> = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
  let gy = abs(gx) - 0.5;
  let tx = floor(gx + 0.5);
  gx = gx - tx;
  var g00: vec2<f32> = vec2<f32>(gx.x, gy.x);
  var g10: vec2<f32> = vec2<f32>(gx.y, gy.y);
  var g01: vec2<f32> = vec2<f32>(gx.z, gy.z);
  var g11: vec2<f32> = vec2<f32>(gx.w, gy.w);
  let norm = 1.79284291400159 - 0.85373472095314 *
      vec4<f32>(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
  g00 = g00 * norm.x;
  g01 = g01 * norm.y;
  g10 = g10 * norm.z;
  g11 = g11 * norm.w;
  let n00 = dot(g00, vec2<f32>(fx.x, fy.x));
  let n10 = dot(g10, vec2<f32>(fx.y, fy.y));
  let n01 = dot(g01, vec2<f32>(fx.z, fy.z));
  let n11 = dot(g11, vec2<f32>(fx.w, fy.w));
  let fade_xy = fade_2(Pf.xy);
  let n_x = mix(vec2<f32>(n00, n01), vec2<f32>(n10, n11), vec2<f32>(fade_xy.x));
  let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
  return 2.3 * n_xy;
}

let high_frequency = 0.3;
let low_frequency = 0.2;
let hf_speed = 0.2;
let lf_speed = 0.12;
let lf_percentage = 0.6;

fn noise(position: vec2<f32>) -> f32 {    
    let lf_position = position * low_frequency;
    let hf_position = position * high_frequency;

    let lf_offset = vec2<f32>(-0.8 * globals.time, -1.2 * globals.time) * lf_speed;
    let hf_offset = vec2<f32>(-1.1 * globals.time, -0.9 * globals.time) * hf_speed;

    let lf_noise = (perlin_noise(lf_position + lf_offset) + 1.0) / 2.0;

    let hf_static_noise = perlin_noise(hf_position);
    let hf_dynamic_noise = perlin_noise(hf_position + hf_offset);
    let hf_noise_raw = (hf_static_noise + hf_dynamic_noise) / 2.0;
    var hf_noise_mirrored: f32;
    if hf_noise_raw >= 0.0 {
        hf_noise_mirrored = hf_noise_raw;
    } else {
        hf_noise_mirrored = -hf_noise_raw * 0.5;
    };
    let hf_noise = smoothstep(0.0, 1.0, hf_noise_mirrored);    

    return lf_noise * lf_percentage + hf_noise * (1.0 - lf_percentage);
}

// structs

struct FogMaterial {
    color: vec4<f32>,
};

struct Vertex {
    @location(0) position: vec3<f32>,
};

struct Fragment {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position_2d: vec2<f32>,
};

// bind group

@group(1) @binding(0)
var<uniform> material: FogMaterial;

// vertex

let height = 2.0;

@vertex
fn vertex(in: Vertex) -> Fragment {
    let position_2d = in.position.xz;
    let noise = noise(position_2d);
    let position = vec3<f32>(position_2d.x, noise * height, position_2d.y);
    
    let clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));

    return Fragment(clip_position, position_2d);
}

// fragment

// let color_1 = vec3<f32>(0.15, 0.35, 0.35);
// let color_2 = vec3<f32>(0.0, 0.0, 0.0);
let color_1 = vec3<f32>(0.35, 0.15, 0.25);
let color_2 = vec3<f32>(0.0, 0.0, 0.0);

@fragment
fn fragment(in: Fragment) -> @location(0) vec4<f32> {
    let noise = noise(in.position_2d);
    return vec4<f32>(mix(color_2, color_1, noise), 0.99);
}
