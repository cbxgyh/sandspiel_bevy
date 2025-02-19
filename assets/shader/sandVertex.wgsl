struct VertexInput {
    @location(0) position : vec2<f32>
};

struct VertexOutput {
    @location(0) uv : vec2<f32>,
    @builtin(position) position : vec4<f32>,
};


@vertex
fn main(input : VertexInput) -> VertexOutput {
    var output : VertexOutput;
    output.uv = input.position;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    return output;
}