// 顶点着色器，用于处理顶点位置并计算出片段着色器中需要使用的纹理坐标。
// 它计算了当前片段（或像素）相对于其左、右、上、下邻近像素的坐标（vL, vR, vT, vB）。
// 这些信息通常用于模拟过程，如流体动力学、纹理处理或计算涡度等。

// 顶点位置，传递到顶点着色器的属性变量。通常这是标准的顶点坐标，范围是 [-1, 1]，表示屏幕空间中的位置。
struct VertexInput {
    @location(0) aPosition : vec2<f32>;
};

// 输出结构体，包含传递到片段着色器的变量
struct VertexOutput {
    @builtin(position) Position : vec4<f32>;
    @location(0) vUv : vec2<f32>;
    @location(1) vL : vec2<f32>;
    @location(2) vR : vec2<f32>;
    @location(3) vT : vec2<f32>;
    @location(4) vB : vec2<f32>;
};

// 纹素大小，表示每个纹理元素（Texel）的大小，通常是 1.0 / textureWidth 和 1.0 / textureHeight，用于计算纹理坐标的偏移。
@group(0) @binding(1) var<uniform> texelSize : vec2<f32>;


@vertex
fn main(input : VertexInput) -> VertexOutput {
    var output : VertexOutput;

    // 计算纹理坐标 (vUv):
    // 顶点坐标 aPosition 通常在 [-1, 1] 范围内，表示裁剪空间中的坐标。为了将其转换为纹理坐标系统，使用以下公式：
    // aPosition * 0.5 将坐标范围从 [-1, 1] 缩放到 [ -0.5, 0.5]。
    // + 0.5 将范围从 [ -0.5, 0.5] 平移到 [0.0, 1.0]，这是纹理坐标的标准范围。
    // vUv 是最终的纹理坐标，在 [0, 1] 范围内，用于片段着色器中的纹理采样。
    output.vUv = input.aPosition * 0.5 + 0.5;

    // 计算邻近像素的纹理坐标:
    // vL, vR, vT, vB 分别表示当前纹理坐标 vUv 左、右、上、下相邻像素的纹理坐标。
    // vL = vUv - vec2(texelSize.x, 0.0) 将纹理坐标沿 x 轴偏移一个纹素大小，得到左侧相邻像素的坐标。
    // vR = vUv + vec2(texelSize.x, 0.0) 将纹理坐标沿 x 轴偏移一个纹素大小，得到右侧相邻像素的坐标。
    // vT = vUv + vec2(0.0, texelSize.y) 将纹理坐标沿 y 轴偏移一个纹素大小，得到上方相邻像素的坐标。
    // vB = vUv - vec2(0.0, texelSize.y) 将纹理坐标沿 y 轴偏移一个纹素大小，得到下方相邻像素的坐标。
    // 这些计算的目的是为片段着色器提供邻近像素的坐标信息，以便在片段着色器中进行邻域采样。
    // 这些坐标通常用于像素级的差分计算或流体模拟中的涡度计算等场景。
    output.vL = output.vUv - vec2<f32>(texelSize.x, 0.0);
    output.vR = output.vUv + vec2<f32>(texelSize.x, 0.0);
    output.vT = output.vUv + vec2<f32>(0.0, texelSize.y);
    output.vB = output.vUv - vec2<f32>(0.0, texelSize.y);

    // 计算最终的顶点位置:
    // 最后，通过 Position 将输入的顶点坐标 aPosition 转换为裁剪空间坐标。
    // 由于顶点坐标已经是裁剪空间坐标，所以 z 和 w 分量被设置为 0.0 和 1.0，这表示位置在二维平面内。
    output.Position = vec4<f32>(input.aPosition, 0.0, 1.0);

    return output;
}