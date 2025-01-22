@group(1) @binding(0) var data: texture_2d<f32>;
@group(1) @binding(1) var backBuffer: texture_2d<f32>;



struct SandUniform{
    t: f32,
    dpi: f32,
    resolution: vec2<f32>,
    isSnapshot: bool
}
@group(0) @binding(2) var<uniform> value: SandUniform;

// 假设我们有以下 GLSL 库函数的 WGSL 实现，这里仅为示例，可能需要更复杂的实现
// 你可能需要找到相应的 WGSL 库或自行实现这些函数
fn hsv2rgb(hsv: vec3<f32>) -> vec3<f32> {
    // 实现 hsv 到 rgb 的转换
    // 示例实现，可能不准确
    let c = hsv.z * hsv.y;
    let x = c * (1.0 - abs(mod(hsv.x * 6.0, 2.0) - 1.0));
    let m = hsv.z - c;
    if (hsv.x < 1.0 / 6.0) {
        return vec3<f32>(c + m, x + m, m);
    } else if (hsv.x < 2.0 / 6.0) {
        return vec3<f32>(x + m, c + m, m);
    } else if (hsv.x < 3.0 / 6.0) {
        return vec3<f32>(m, c + m, x + m);
    } else if (hsv.x < 4.0 / 6.0) {
        return vec3<f32>(m, x + m, c + m);
    } else if (hsv.x < 5.0 / 6.0) {
        return vec3<f32>(x + m, m, c + m);
    } else {
        return vec3<f32>(c + m, m, x + m);
    }
}


fn snoise3(p: vec3<f32>, t: f32) -> f32 {
    // 实现 3D Simplex 噪声，需要更复杂的实现，这里仅为占位符
    return 0.0;
}


fn snoise2(p: vec2<f32>) -> f32 {
    // 实现 2D Simplex 噪声，需要更复杂的实现，这里仅为占位符
    return 0.0;
}


fn random(p: vec2<f32>) -> f32 {
    // 实现随机函数，需要更复杂的实现，这里仅为占位符
    return 0.0;
}


@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let textCoord: vec2<f32> = ((uv * vec2<f32>(0.5, -0.5)) + vec2<f32>(0.5)).yx;
    let data: vec4<f32> = textureSample(data, sampler(data), textCoord);
    let type_val: i32 = i32((data.r * 255.0) + 0.1);
    var hue: f32 = 0.0;
    var saturation: f32 = 0.6;
    var lightness: f32 = 0.3 + data.g * 0.5;
    let noise: f32 = snoise3(vec3<f32>(floor(uv * value.resolution / value.dpi), value.t * 0.05));
    var a: f32 = 1.0;


    if (type_val == 0) {
        hue = 0.0;
        saturation = 0.1;
        lightness = 0.1;
        a = 0.1;
        if (value.isSnapshot) {
            saturation = 0.05;
            lightness = 1.01;
            a = 1.0;
        }
    } else if (type_val == 1) {
        hue = 0.1;
        saturation = 0.1;
        lightness = 0.4;
    } else if (type_val == 2) {
        hue = 0.1;
        saturation = 0.5;
        lightness += 0.3;
    } else if (type_val == 3) { // water
        hue = 0.6;
        lightness = 0.7 + data.g * 0.25 + noise * 0.1;
        let polarity: i32 = i32(mod(data.g * 255.0, 2.0) + 0.1);
        if (polarity == 0) {
            lightness += 0.01;
        }
    } else if (type_val == 4) { // gas
        hue = 0.0;
        lightness += 0.4;
        saturation = 0.2 + (data.b * 1.5);
    } else if (type_val == 5) { // clone
        hue = 0.9;
        saturation = 0.3;
    } else if (type_val == 6) { // fire
        hue = (data.g * 0.1);
        saturation = 0.7;
        lightness = 0.7 + (data.g * 0.3) + ((noise + 0.8) * 0.5);
        if (value.isSnapshot) {
            lightness -= 0.2;
        }
    } else if (type_val == 7) { // wood
        hue = (data.g * 0.1);
        saturation = 0.3;
        lightness = 0.3 + data.g * 0.3;
    } else if (type_val == 8) { // lava
        hue = (data.g * 0.1);
        lightness = 0.7 + data.g * 0.25 + noise * 0.1;
    } else if (type_val == 9) { // ice
        hue = 0.6;
        saturation = 0.4;
        lightness = 0.7 + data.g * 0.5;
    } else if (type_val == 10) { // sink
        hue = 0.9;
        saturation = 0.4;
        lightness = 1.0;
    } else if (type_val == 11) { // plant
        hue = 0.4;
        saturation = 0.4;
    } else if (type_val == 12) { // acid
        hue = 0.18;
        saturation = 0.9;
        lightness = 0.8 + data.g * 0.2 + noise * 0.05;
    } else if (type_val == 13) { // stone
        hue = -0.4 + (data.g * 0.5);
        saturation = 0.1;
        // lightness = 0.2 + data.g * 0.5;
    } else if (type_val == 14) { // dust
        hue = (data.g * 2.0) + value.t * 0.0008;
        saturation = 0.4;
        lightness = 0.8;
    } else if (type_val == 15) { // mite
        hue = 0.8;
        saturation = 0.9;
        lightness = 0.8;
    } else if (type_val == 16) { // oil
        hue = (data.g * 5.0) + value.t * 0.008;
        saturation = 0.2;
        lightness = 0.3;
    } else if (type_val == 17) { // Rocket
        hue = 0.0;
        saturation = 0.4 + data.b;
        lightness = 0.9;
    } else if (type_val == 18) { // fungus
        hue = (data.g * 0.15) - 0.1;
        saturation = (data.g * 0.8) - 0.05;
        // (data.g * 0.00);
        lightness = 1.5 - (data.g * 0.2);
    } else if (type_val == 19) { // seed/flower
        hue = fract(fract(data.b * 2.0) * 0.5) - 0.3;
        saturation = 0.7 * (data.g + 0.4) + data.b * 0.2;
        lightness = 0.9 * (data.g + 0.9);
    }


    if (value.isSnapshot == false) {
        lightness *= (0.975 + snoise2(floor(uv * value.resolution / value.dpi)) * 0.025);
    }


    let color: vec3<f32> = hsv2rgb(vec3<f32>(hue, saturation, lightness));
    return vec4<f32>(color, a);
}