/// Endpoint build a WGPU array buffer
pub mod points;
/// Endpoint:  Multiple lines laid out in a vertex and index buffer
// Enabling a single draw call to render all.
pub mod polylines;

use bytemuck::Pod;
use bytemuck::Zeroable;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq)]
/// CPU side representation of a Vertex shader primitve.
pub struct Vertex {
    /// The coords of the vertex
    pub pos: [f32; 2],
}

impl Vertex {
    /// description a `wgpu::VertexState parameter`
    /// The layout in memory of the vertex array.
    #[must_use]
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }
}
