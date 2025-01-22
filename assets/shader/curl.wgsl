@group(0) @binding(0) var uVelocity: texture_2d<f32>;

@fragment
fn main(
    @location(0) vUv: vec2<f32>,
    @location(1) vL: vec2<f32>,
    @location(2) vR: vec2<f32>,
    @location(3) vT: vec2<f32>,
    @location(4) vB: vec2<f32>
) -> @location(0) vec4<f32> {
    // 从 uVelocity 纹理中采样相应纹理坐标处的速度分量
    let L: f32 = textureSample(uVelocity, sampler(uVelocity), vL).y;
    let R: f32 = textureSample(uVelocity, sampler(uVelocity), vR).y;
    let T: f32 = textureSample(uVelocity, sampler(uVelocity), vT).x;
    let B: f32 = textureSample(uVelocity, sampler(uVelocity), vB).x;

    // 计算涡度
    let vorticity: f32 = R - L - T + B;

    // 将涡度作为红色分量输出，其余通道为 0.0 和 1.0（不透明）
    return vec4<f32>(vorticity, 0.0, 0.0, 1.0);
}