@group(1) @binding(0) var uTexture: texture_2d<f32>;
@group(1) @binding(1) var uSampler: sampler;
//@group 和 @binding 是用于在 WebGPU 中指定资源绑定信息的属性。@group 定义了资源所属的绑定组，@binding 表示该资源在绑定组内的具体位置。
//uTexture 是一个二维浮点纹理，存储了需要处理的图像数据。
//uSampler 是一个采样器，用于对 uTexture 进行采样操作，它决定了采样的方式（如过滤模式、寻址模式等）。

struct VertexOutput {
    @location(0) vUv: vec2<f32>
};

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    // 从纹理中采样颜色，并对纹理坐标进行变换，反转 y 轴并乘以 0.1 使初始颜色变暗
    var color = textureSample(uTexture, uSampler, vec2<f32>(1.0 - input.vUv.y, input.vUv.x)).rgb * 0.1;
    // 进一步降低亮度
//    将之前采样并处理得到的颜色的 RGB 分量再次乘以 0.5，进一步降低颜色的亮度。
    color *= 0.5;
    // 将颜色分量限制在 0.9 以下
    color = min(color, vec3<f32>(0.9));
    // 反转颜色
    color = vec3<f32>(1.0) - color;
    // 对颜色分量进行加权调整
    color *= vec3<f32>(0.95, 0.9, 0.9);
    // 输出最终颜色，alpha 通道为 1.0
    return vec4<f32>(color, 1.0);
}