@group(1) @binding(0) var uVelocity: texture_2d<f32>;
@group(1) @binding(1) var uSampler: sampler;

struct VertexOutput {
    @location(0) vUv: vec2<f32>,
    @location(1) vL: vec2<f32>,
    @location(2) vR: vec2<f32>,
    @location(3) vT: vec2<f32>,
    @location(4) vB: vec2<f32>,
};

fn sampleVelocity(uv: vec2<f32>) -> vec2<f32> {
    var multiplier: vec2<f32> = vec2<f32>(1.0, 1.0);
    var result_uv = uv;
    if (uv.x < 0.0) {
        result_uv.x = 0.0;
        multiplier.x = -1.0;
    }
    if (uv.x > 1.0) {
        result_uv.x = 1.0;
        multiplier.x = -1.0;
    }
    if (uv.y < 0.0) {
        result_uv.y = 0.0;
        multiplier.y = -1.0;
    }
    if (uv.y > 1.0) {
        result_uv.y = 1.0;
        multiplier.y = -1.0;
    }
    return multiplier * textureSample(uVelocity, uSampler, result_uv).xy;
}

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    var L: f32 = sampleVelocity(input.vL).x;
    var R: f32 = sampleVelocity(input.vR).x;
    var T: f32 = sampleVelocity(input.vT).y;
    var B: f32 = sampleVelocity(input.vB).y;
    var div: f32 = 0.5 * (R - L + T - B);
    return vec4<f32>(div, 0.0, 0.0, 1.0);
}