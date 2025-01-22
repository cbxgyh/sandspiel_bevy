@group(1) @binding(0) var uTarget: texture_2d<f32>;
@group(1) @binding(1) var uSampler: sampler;

struct SplatUniform{
   aspect_ratio: f32,
   color: vec3<f32>,
   point: vec2<f32>,
   radius: f32
}
@group(1) @binding(2) var<uniform> splat_value: SplatUniform;


struct VertexOutput {
    @location(0) vUv: vec2<f32>
};

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    var p = input.vUv - splat_value.point;
    p.x *= splat_value.aspect_ratio;
    var splat = exp(-dot(p, p) / splat_value.radius) * splat_value.color;
    var base = textureSample(uTarget, uSampler, input.vUv).xyz;
    return vec4<f32>(base + splat, 1.0);
}