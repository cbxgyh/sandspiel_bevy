@group(0) @binding(0) var uVelocity : texture_2d<f32>;
@group(0) @binding(1) var uSource : texture_2d<f32>;
@group(0) @binding(2) var uWind : texture_2d<f32>;
@group(0) @binding(3) var uSampler : sampler;

@group(0) @binding(4) var<uniform> advection_value: AdvectionUniform;
struct AdvectionUniform {
    texel_size : vec2<f32>,
    dt : f32,
    dissipation : f32
};
struct VertexOutput {
    @location(0) vUv : vec2<f32>;
};


@fragment
fn main(input : VertexOutput) -> @location(0) vec4<f32> {
    // 坐标计算
    var coord = input.vUv - advection_value.dt * textureSample(uVelocity, uSampler, input.vUv).xy * advection_value.texel_size;
    // 密度计算
    var density = textureSample(uWind, uSampler, input.vUv).w * 1.0;
    if (density > 0.99) {
        density = 0.0;
    }
    var newSource = textureSample(uSource, uSampler, coord);
    var result = advection_value.dissipation * (newSource + vec4<f32>(density));
    result.a = 1.0;
    return result;
}