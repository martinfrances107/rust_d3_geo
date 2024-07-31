/// Endpoint build a WGPU array buffer
pub mod points;
pub mod polylines;

use bytemuck::Pod;
use bytemuck::Zeroable;

// NB PartialEq is for testing only.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq)]
/// CPU side representation of a Vertex shader primitve.
pub struct Vertex {
    // The coords of the vertex
    /// TODO can I make the [f32;2]
    pub pos: [f32; 3],
}

impl Vertex {
    /// description a `wgpu::VertexState parameter`
    /// The layout in memory of the vertex array.
    #[must_use]
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}
