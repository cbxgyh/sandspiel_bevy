@group(1) @binding(0) var uVelocity: texture_2d<f32>;
@group(1) @binding(1) var uSampler: sampler;

//@group 和 @binding 是 WGSL 中用于资源绑定的属性。@group 表示资源所属的绑定组，@binding 表示在该绑定组内的具体绑定位置。
  //uVelocity 是一个二维浮点纹理，用于存储流体的速度场数据。
  //uSampler 是一个采样器，用于对 uVelocity 纹理进行采样操作。
@fragment
fn main(
    @location(0) vUv: vec2<f32>,
    @location(1) vL: vec2<f32>,
    @location(2) vR: vec2<f32>,
    @location(3) vT: vec2<f32>,
    @location(4) vB: vec2<f32>
) -> @location(0) vec4<f32> {
    // 从 uVelocity 纹理中采样相应纹理坐标处的速度分量
//    vUv：二维浮点向量，表示当前片段的纹理坐标。
      //vL、vR、vT、vB：分别表示当前片段左侧、右侧、上方和下方相邻位置的纹理坐标。
//      这些坐标通常由顶点着色器计算并传递给片段着色器。
    let L: f32 = textureSample(uVelocity, uSampler, vL).y;
    let R: f32 = textureSample(uVelocity, uSampler, vR).y;
    let T: f32 = textureSample(uVelocity, uSampler, vT).x;
    let B: f32 = textureSample(uVelocity, uSampler, vB).x;

    // 计算涡度
    let vorticity: f32 = R - L - T + B;

    // 将涡度作为红色分量输出，其余通道为 0.0 和 1.0（不透明）
    return vec4<f32>(vorticity, 0.0, 0.0, 1.0);
}