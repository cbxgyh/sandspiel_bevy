@group(0) @binding(0) var t: f32;
@group(0) @binding(1) var dpi: f32;
@group(0) @binding(2) var resolution: vec2<f32>;
@group(0) @binding(3) var isSnapshot: bool;
@group(0) @binding(4) var backBuffer: texture_2d<f32>;
@group(0) @binding(5) var data: texture_2d<f32>;
@group(0) @binding(6) var backBufferSampler: sampler;
@group(0) @binding(7) var dataSampler: sampler;


struct VertexOutput {
    @location(0) uv: vec2<f32>;
};


fn hsv2rgb(hsv: vec3<f32>) -> vec3<f32> {
    // 这里需要实现 hsv 到 rgb 的转换函数
    // 以下是一个简单的实现，可能不准确，可根据实际需求完善
    let c = hsv.z * hsv.y;
    let x = c * (1.0 - abs(mod(hsv.x * 6.0, 2.0) - 1.0));
    let m = hsv.z - c;
    var rgb: vec3<f32>;
    if (hsv.x < 1.0 / 6.0) {
        rgb = vec3<f32>(c, x, 0.0) + m;
    } else if (hsv.x < 2.0 / 6.0) {
        rgb = vec3<f32>(x, c, 0.0) + m;
    } else if (hsv.x < 3.0 / 6.0) {
        rgb = vec3<f32>(0.0, c, x) + m;
    } else if (hsv.x < 4.0 / 6.0) {
        rgb = vec3<f32>(0.0, x, c) + m;
    } else if (hsv.x < 5.0 / 6.0) {
        rgb = vec3<f32>(x, 0.0, c) + m;
    } else {
        rgb = vec3<f32>(c, 0.0, x) + m;
    }
    return rgb;
}


fn snoise3(p: vec3<f32>) -> f32 {
    // 这里需要实现三维 Simplex 噪声函数
    // 以下是一个简单的占位实现，可根据实际需求完善
    return 0.0;
}


fn snoise2(p: vec2<f32>) -> f32 {
    // 这里需要实现二维 Simplex 噪声函数
    // 以下是一个简单的占位实现，可根据实际需求完善
    return 0.0;
}


fn random(p: vec2<f32>) -> f32 {
    // 这里需要实现随机函数
    // 以下是一个简单的占位实现，可根据实际需求完善
    return 0.0;
}


@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec3<f32>;
    // float r = abs(sin(t / 25.));
    // if (length(input.uv) < r && length(input.uv) > r - 0.1) {
    // color = hsv2rgb(vec3(sin(t * 0.01), 0.5, 0.5));


    var textCoord = ((input.uv * vec2<f32>(0.5, -0.5)) + vec2<f32>(0.5)).yx;


    var dataSample = textureSample(data, dataSampler, textCoord);
    var type: i32 = i32((dataSample.r * 255.0) + 0.1);
    var hue: f32 = 0.0;
    var saturation: f32 = 0.6;
    var lightness: f32 = 0.3 + dataSample.g * 0.5;
    var noise = snoise3(vec3<f32>(floor(input.uv * resolution / dpi), t * 0.05));
    var a: f32 = 1.0;


    if (type == 0) {
        hue = 0.0;
        saturation = 0.1;
        lightness = 0.1;
        a = 0.1;
        if (isSnapshot) {
            saturation = 0.05;
            lightness = 1.01;
            a = 1.0;
        }
    } else if (type == 1) {
        hue = 0.1;
        saturation = 0.1;
        lightness = 0.4;
    } else if (type == 2) {
        hue = 0.1;
        saturation = 0.5;
        lightness += 0.3;
    } else if (type == 3) { // water
        hue = 0.6;
        lightness = 0.7 + dataSample.g * 0.25 + noise * 0.1;
        var polarity: i32 = i32(mod(dataSample.g * 255.0, 2.0) + 0.1);
        if (polarity == 0) {
            lightness += 0.01;
        }


    } else if (type == 4) { // gas
        hue = 0.0;
        lightness += 0.4;
        saturation = 0.2 + (dataSample.b * 1.5);
    } else if (type == 5) { // clone
        hue = 0.9;
        saturation = 0.3;
    } else if (type == 6) { // fire
        hue = (dataSample.g * 0.1);
        saturation = 0.7;


        lightness = 0.7 + (dataSample.g * 0.3) + ((noise + 0.8) * 0.5);
        if (isSnapshot) {
            lightness -= 0.2;
        }
    } else if (type == 7) { // wood
        hue = (dataSample.g * 0.1);
        saturation = 0.3;
        lightness = 0.3 + dataSample.g * 0.3;
    } else if (type == 8) { // lava
        hue = (dataSample.g * 0.1);
        lightness = 0.7 + dataSample.g * 0.25 + noise * 0.1;
    } else if (type == 9) { // ice
        hue = 0.6;
        saturation = 0.4;
        lightness = 0.7 + dataSample.g * 0.5;
    } else if (type == 10) { // sink
        hue = 0.9;
        saturation = 0.4;
        lightness = 1.0;
    } else if (type == 11) { // plant
        hue = 0.4;
        saturation = 0.4;
    } else if (type == 12) { // acid
        hue = 0.18;
        saturation = 0.9;
        lightness = 0.8 + dataSample.g * 0.2 + noise * 0.05;
    } else if (type == 13) { // stone
        hue = -0.4 + (dataSample.g * 0.5);
        saturation = 0.1;
        // lightness = 0.2 + dataSample.g * 0.5;
    } else if (type == 14) { // dust
        hue = (dataSample.g * 2.0) + t * 0.0008;
        saturation = 0.4;
        lightness = 0.8;
    } else if (type == 15) { // mite
        hue = 0.8;
        saturation = 0.9;
        lightness = 0.8;
    } else if (type == 16) { // oil
        hue = (dataSample.g * 5.0) + t * 0.008;


        saturation = 0.2;
        lightness = 0.3;
    } else if (type == 17) { // Rocket
        hue = 0.0;
        saturation = 0.4 + dataSample.b;
        lightness = 0.9;
    } else if (type == 18) { // fungus
        hue = (dataSample.g * 0.15) - 0.1;
        saturation = (dataSample.g * 0.8) - 0.05;


        // (dataSample.g * 0.00);
        lightness = 1.5 - (dataSample.g * 0.2);
    } else if (type == 19) { // seed/flower


        hue = fract(fract(dataSample.b * 2.0) * 0.5) - 0.3;
        saturation = 0.7 * (dataSample.g + 0.4) + dataSample.b * 0.2;
        lightness = 0.9 * (dataSample.g + 0.9);
    }
    if (!isSnapshot) {
        lightness *= (0.975 + snoise2(floor(input.uv * resolution / dpi)) * 0.025);
    }
    color = hsv2rgb(vec3<f32>(hue, saturation, lightness));
    return vec4<f32>(color, a);
}