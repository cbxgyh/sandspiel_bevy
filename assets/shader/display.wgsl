@group(1) @binding(0) var uTexture: texture_2d<f32>;
@group(1) @binding(1) var uSampler: sampler;

struct VertexOutput {
    @location(0) vUv: vec2<f32>
};

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    // 从纹理中采样颜色，并对纹理坐标进行变换，反转 y 轴并乘以 0.1 使初始颜色变暗
    var color = textureSample(uTexture, uSampler, vec2<f32>(1.0 - input.vUv.y, input.vUv.x)).rgb * 0.1;
    // 进一步降低亮度
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