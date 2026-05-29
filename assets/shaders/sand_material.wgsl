#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct SandMaterialParams {
    time: f32,
    width: f32,
    height: f32,
    _padding: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var cells_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var cells_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var fluid_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var fluid_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> params: SandMaterialParams;

fn hash21(p: vec2<f32>) -> f32 {
    let q = fract(p * vec2<f32>(0.1031, 0.1030));
    let k = q + dot(q, q.yx + 33.33);
    return fract((k.x + k.y) * k.x);
}

fn hsv2rgb(c: vec3<f32>) -> vec3<f32> {
    let k = vec4<f32>(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p = abs(fract(c.xxx + k.xyz) * 6.0 - k.www);
    return c.z * mix(k.xxx, clamp(p - k.xxx, vec3<f32>(0.0), vec3<f32>(1.0)), c.y);
}

fn particle_color(kind: i32, data: vec4<f32>, world: vec2<f32>) -> vec4<f32> {
    let noise = hash21(floor(world) + vec2<f32>(params.time * 11.0, params.time * 7.0));
    var hue = 0.0;
    var saturation = 0.6;
    var lightness = 0.3 + data.g * 0.5;
    var alpha = 1.0;

    if kind == 0 {
        alpha = 0.0;
    } else if kind == 1 {
        hue = 0.1;
        saturation = 0.1;
        lightness = 0.4;
    } else if kind == 2 {
        hue = 0.1;
        saturation = 0.5;
        lightness += 0.3;
    } else if kind == 3 {
        hue = 0.6;
        saturation = 0.5;
        lightness = 0.7 + data.g * 0.25 + noise * 0.1;
        if i32(data.g * 255.0) % 2 == 0 {
            lightness += 0.01;
        }
    } else if kind == 4 {
        hue = 0.0;
        lightness += 0.4;
        saturation = 0.2 + (data.b * 1.5);
    } else if kind == 5 {
        hue = 0.9;
        saturation = 0.3;
    } else if kind == 6 {
        hue = data.g * 0.1;
        saturation = 0.7;
        lightness = 0.7 + (data.g * 0.3) + ((noise + 0.8) * 0.5);
    } else if kind == 7 {
        hue = data.g * 0.1;
        saturation = 0.3;
        lightness = 0.3 + data.g * 0.3;
    } else if kind == 8 {
        hue = data.g * 0.1;
        lightness = 0.7 + data.g * 0.25 + noise * 0.1;
    } else if kind == 9 {
        hue = 0.6;
        saturation = 0.4;
        lightness = 0.7 + data.g * 0.5;
    } else if kind == 11 {
        hue = 0.4;
        saturation = 0.4;
    } else if kind == 12 {
        hue = 0.18;
        saturation = 0.9;
        lightness = 0.8 + data.g * 0.2 + noise * 0.05;
    } else if kind == 13 {
        hue = -0.4 + (data.g * 0.5);
        saturation = 0.1;
    } else if kind == 14 {
        hue = (data.g * 2.0) + params.time * 0.0008;
        saturation = 0.4;
        lightness = 0.8;
    } else if kind == 15 {
        hue = 0.8;
        saturation = 0.9;
        lightness = 0.8;
    } else if kind == 16 {
        hue = (data.g * 5.0) + params.time * 0.008;
        saturation = 0.2;
        lightness = 0.3;
    } else if kind == 17 {
        hue = 0.0;
        saturation = 0.4 + data.b;
        lightness = 0.9;
    } else if kind == 18 {
        hue = (data.g * 0.15) - 0.1;
        saturation = (data.g * 0.8) - 0.05;
        lightness = 1.5 - (data.g * 0.2);
    } else if kind == 19 {
        hue = fract(fract(data.b * 2.0) * 0.5) - 0.3;
        saturation = 0.7 * (data.g + 0.4) + data.b * 0.2;
        lightness = 0.9 * (data.g + 0.9);
    }

    lightness *= 0.975 + (hash21(world) * 0.05 - 0.025);
    return vec4<f32>(hsv2rgb(vec3<f32>(hue, saturation, lightness)), alpha);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let cell = textureSample(cells_texture, cells_sampler, uv);
    let fluid = textureSample(fluid_texture, fluid_sampler, uv);
    let world = vec2<f32>(uv.x * params.width, uv.y * params.height);

    let vx = (fluid.r * 255.0 - 126.0) / 126.0;
    let vy = (fluid.g * 255.0 - 126.0) / 126.0;
    let density = fluid.b;
    let pressure = fluid.a;

    var background = vec3<f32>(0.97, 0.94, 0.91);
    background -= vec3<f32>(density * 0.10, density * 0.15, density * 0.18);
    background += vec3<f32>(abs(vx) * 0.03, pressure * 0.03, abs(vy) * 0.04);
    background = clamp(background, vec3<f32>(0.05), vec3<f32>(1.0));

    let kind = i32(cell.r * 255.0 + 0.1);
    let sand = particle_color(kind, cell, world);
    return vec4<f32>(mix(background, sand.rgb, sand.a), 1.0);
}
