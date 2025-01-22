@group(1) @binding(0) var uPressure: texture_2d<f32>;
@group(1) @binding(1) var uVelocity: texture_2d<f32>;
@group(1) @binding(2) var uWind: texture_2d<f32>;
@group(1) @binding(3) var uCells: texture_2d<f32>;

// 边界处理函数，确保纹理坐标在 [0.0, 1.0] 范围内
fn boundary(uv: vec2<f32>) -> vec2<f32> {
    return clamp(uv, 0.0, 1.0);
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
    let L: f32 = textureSample(uPressure, sampler(uPressure), boundary(vL)).x;
    let R: f32 = textureSample(uPressure, sampler(uPressure), boundary(vR)).x;
    let T: f32 = textureSample(uPressure, boundary(vT)).x;
    let B: f32 = textureSample(uPressure, boundary(vB)).x;

    // 采样速度、风场和单元信息
    let velocity: vec2<f32> = textureSample(uVelocity, sampler(uVelocity), vUv).xy;
    let wind: vec2<f32> = textureSample(uWind, sampler(uWind), vUv).xy;
    let cell: vec2<f32> = textureSample(uCells, sampler(uCells), vUv).xy;

    // 速度更新
    velocity = velocity - vec2<f32>(R - L, T - B);
    velocity.yx = velocity.yx + wind * -25.0;

    // 类型判断和速度调整
    let type_val: i32 = i32((cell.r * 255.0) + 0.1);
    if (type_val == 1 || type_val == 5) {
        velocity = vec2<f32>(0.0, 0.0);
    } else if (type_val!= 0 && type_val!= 4 && type_val!= 6) {
        velocity = velocity * 0.95;
    }

    // 输出结果
    return vec4<f32>(velocity, 0.0, 1.0);
}