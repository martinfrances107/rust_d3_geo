// @vertex
// fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
//     // let x = f32(i32(in_vertex_index) - 1);
//     // let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
//     let x =
//     return vec4<f32>(x, y, 0.0, 1.0);
// }

// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {

  // Convert points in the space form
  //
  // x: -180 ->180
  // y: -90 -> 90
  //
  // into clip space -1 to 1!!!
  //
  // TODO Use the GPU properly -- use a SIMD like transform!
  // or CPU side adjust the scale_set and tranfrom_set()
  // so that every thing automatically comes out into clip space.
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position.x / 300. , model.position.y / 250.  , 0.0, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
