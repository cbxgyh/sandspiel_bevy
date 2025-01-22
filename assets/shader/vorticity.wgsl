@group(0) @binding(0) var uVelocity: texture_2d<f32>;
@group(0) @binding(1) var uCurl: texture_2d<f32>;
@group(0) @binding(2) var<uniform> value: VorticityUniform;

struct VorticityUniform{
    curl: f32,
    dt: f32
}

@fragment
fn main(@location(0) vUv: vec2<f32>, @location(1) vT: vec2<f32>, @location(2) vB: vec2<f32>) -> @location(0) vec4<f32> {
    // 从 uCurl 纹理中采样相应纹理坐标处的卷曲值
    let T: f32 = textureSample(uCurl, sampler(uCurl), vT).x;
    let B: f32 = textureSample(uCurl, vB).x;
    let C: f32 = textureSample(uCurl, sampler(uCurl), vUv).x;
    // 计算卷曲力
    let force: vec2<f32> = vec2<f32>(abs(T) - abs(B), 0.0);
    force = force * (1.0 / length(force + 0.00001)) * value.curl * C;

    // 从 uVelocity 纹理中采样当前像素的速度
    let vel: vec2<f32> = textureSample(uVelocity, sampler(uVelocity), vUv).xy;

    // 更新速度场并输出结果
    let result: vec2<f32> = vel + force * value.dt;
    return vec4<f32>(result, 0.0, 1.0);
}