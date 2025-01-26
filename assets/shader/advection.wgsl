@group(1) @binding(0) var uVelocity : texture_2d<f32>;
@group(1) @binding(1) var uSource : texture_2d<f32>;
@group(1) @binding(2) var uWind : texture_2d<f32>;
@group(1) @binding(3) var uSampler : sampler;

//@group 和 @binding 是 WGSL 中用于资源绑定的属性。@group 表示资源所属的绑定组，@binding 表示在该绑定组内的具体绑定位置。
  //uVelocity、uSource 和 uWind 分别是 2D 浮点纹理，用于存储速度场、源数据和风力数据。
  //uSampler 是一个采样器，用于对纹理进行采样操作。
  //advection_value 是一个统一变量（uniform），类型为 AdvectionUniform，用于传递平流相关的参数。
  //统一变量结构体

@group(1) @binding(4) var<uniform> advection_value: AdvectionUniform;

//AdvectionUniform 是一个结构体，包含三个成员：
  //texel_size：二维浮点向量，表示纹理中每个纹素的大小。
  //dt：浮点数，表示时间步长。
  //dissipation：浮点数，表示消散系数，用于控制流体在平流过程中的消散程度。
struct AdvectionUniform {
    texel_size : vec2<f32>,
    dt : f32,
    dissipation : f32
};

//VertexOutput 是一个结构体，用于存储顶点着色器传递给片段着色器的输出数据。
  //vUv 是一个二维浮点向量，表示纹理坐标，@location(0) 表示该变量在输出中的位置。
struct VertexOutput {
    @location(0) vUv : vec2<f32>
};


@fragment
fn main(input : VertexOutput) -> @location(0) vec4<f32> {
    // 坐标计算
//    coord 是根据当前纹理坐标 input.vUv、时间步长 advection_value.dt、
//速度场 uVelocity 和纹素大小 advection_value.texel_size 计算得到的新纹理坐标。通过
//textureSample 函数从 uVelocity 纹理中采样当前位置的速度值，并将其应用到当前纹理坐标上，
//实现平流效果。
    var coord = input.vUv - advection_value.dt * textureSample(uVelocity, uSampler, input.vUv).xy * advection_value.texel_size;
    // 密度计算
//    density 是从 uWind 纹理中采样得到的密度值。这里只取采样结果的 w 分量，并乘以 1.0。
  //如果 density 大于 0.99，则将其置为 0.0，用于过滤掉过高的密度值。
    var density = textureSample(uWind, uSampler, input.vUv).w * 1.0;
    if (density > 0.99) {
        density = 0.0;
    }

//    newSource 是根据新纹理坐标 coord 从 uSource 纹理中采样得到的源数据。
//result 是最终的计算结果，通过将 newSource 和密度值相加，并乘以消散系数
//advection_value.dissipation 得到。
//将 result 的 a 分量（透明度）设置为 1.0，表示完全不透明。
    var newSource = textureSample(uSource, uSampler, coord);
    var result = advection_value.dissipation * (newSource + vec4<f32>(density));
    result.a = 1.0;
    return result;
}