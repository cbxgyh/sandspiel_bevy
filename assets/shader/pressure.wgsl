@group(1) @binding(0) var uPressure: texture_2d<f32>;
@group(1) @binding(1) var uDivergence: texture_2d<f32>;
@group(1) @binding(2) var uSamplerPressure: sampler;
@group(1) @binding(3) var uSamplerDivergence: sampler;
// 边界处理函数，确保纹理坐标在 [0.0, 1.0] 范围内
fn boundary(uv: vec2<f32>) -> vec2<f32> {

    return clamp(uv, vec2<f32>(0.0), vec2<f32>(1.0));
}

@fragment
fn main(
    @location(0) vUv: vec2<f32>,
    @location(1) vL: vec2<f32>,
    @location(2) vR: vec2<f32>,
    @location(3) vT: vec2<f32>,
    @location(4) vB: vec2<f32>
) -> @location(0) vec4<f32> {
    // 对纹理坐标进行边界处理并采样压力
    let L: f32 = textureSample(uPressure, uSamplerPressure, boundary(vL)).x;
    let R: f32 = textureSample(uPressure, uSamplerPressure, boundary(vR)).x;
    let T: f32 = textureSample(uPressure, uSamplerPressure, boundary(vT)).x;
    let B: f32 = textureSample(uPressure, uSamplerPressure, boundary(vB)).x;
    let C: f32 = textureSample(uPressure, uSamplerPressure, vUv).x;

    // 采样散度
    let divergence: f32 = textureSample(uDivergence, uSamplerDivergence, vUv).x;

    // 计算压力
    let pressure: f32 = (L + R + B + T - divergence) * 0.25;

    // 将压力作为红色分量输出，其余通道为 0.0 和 1.0（不透明）
    return vec4<f32>(pressure, 0.0, 0.0, 1.0);
}