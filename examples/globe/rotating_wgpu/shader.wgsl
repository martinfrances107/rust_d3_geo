// Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>,

};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {

  // scale points into clip space -1 to 1!!!
  var out: VertexOutput;
  out.clip_position = vec4<f32>(model.position.x / 256, model.position.y / 256.0  , 0.0, 1.0);
  return out;
}

// Fragment shaders.

@fragment
fn fs_white(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}

@fragment
fn fs_green(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.117, 0.703, 0.242, 1.0);
}
