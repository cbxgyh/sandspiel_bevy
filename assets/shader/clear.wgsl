@group(0) @binding(0) var uTexture: texture_2d<f32>;
@group(0) @binding(1) var uWind: texture_2d<f32>;
@group(0) @binding(2) var<uniform> value: ClearUniform;

struct ClearUniform{
    value: f32,
}

@fragment
fn main(@location(0) vUv: vec2<f32>) -> @location(0) vec4<f32> {
    // 从 uWind 纹理中采样，获取 z 分量作为压力值
    let pressure: f32 = textureLoad(uWind, vec2<i32>(vUv * textureDimensions(uWind)), 0).z;
    // 对压力值进行计算
    pressure = pressure * 512.0;
    pressure = pressure * pressure;
    // 从 uTexture 纹理中采样
    let texColor: vec4<f32> = textureSample(uTexture, sampler(uTexture), vUv);
    // 计算最终的颜色
    let finalColor: vec4<f32> = value.value * (texColor + vec4<f32>(pressure, pressure, pressure, 1.0));
    return finalColor;
}