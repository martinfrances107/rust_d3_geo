// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,

};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {

  // scale points into clip space -1 to 1!!!
  //
  // TODO Use the GPU properly -- use a SIMD like transform?
  var out: VertexOutput;
  out.clip_position = vec4<f32>(model.position.x / 300.0, model.position.y / 250.0  , 0.0, 1.0);
  return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
