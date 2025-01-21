@group(0) @binding(0) var uTexture: texture_2d<f32>;
@group(0) @binding(1) var uPressure: texture_2d<f32>;
@group(0) @binding(2) var uSamplerTexture: sampler;
@group(0) @binding(3) var uSamplerPressure: sampler;

struct VertexOutput {
    @location(0) vUv: vec2<f32>;
};

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    var v = textureSample(uTexture, uSamplerTexture, input.vUv).rg;
    var p = textureSample(uPressure, uSamplerPressure, input.vUv).r;
    var vp = vec3<f32>(v, p);
    vp = max(vp, vec3<f32>(-250.0));
    vp = min(vp, vec3<f32>(250.0));
    vp /= 500.0;
    vp += vec3<f32>(0.5, 0.5, 0.0);
    // v = vec2<f32>(0.5);
    return vec4<f32>(vp, 0.0);
}