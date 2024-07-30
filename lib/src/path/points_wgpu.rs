use core::mem;

// use bytemuck::{Pod, Zeroable};
use geo_types::Coord;

use crate::stream::Stream;

// use super::PointRadiusTrait;
use super::Result;

// NB PartialEq is for testing only.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
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

/// Stream path endpoint: Used when rendering to a HTML Canvas element.
///
/// Wraps a Path2d object, and implements STREAM trait.
#[derive(Clone, Debug, PartialEq)]
pub struct PointsWGPU {
    /// Vertext buffer is a form ready to be shipped to the GPU.
    pub v_buffer: Vec<Vertex>,
}

impl Default for PointsWGPU {
    #[inline]
    fn default() -> Self {
        // A default capacity 200 points!!
        // TODO consider a new(capacity: usize)
        // which allows this to be overrriden.
        Self {
            v_buffer: Vec::with_capacity(200usize),
        }
    }
}

/// Return path2d, blanking the stored value.
///
/// Architecture Discussion:
///
/// I am making the assumption here that in a animation frame
/// repeated calls to [result] will return approximatly
/// the same number of elements.
impl Result for PointsWGPU {
    type Out = Vec<Vertex>;
    #[inline]
    fn result(&mut self) -> Self::Out {
        let mut out = Vec::with_capacity(self.v_buffer.capacity());
        mem::swap(&mut out, &mut self.v_buffer);
        out
    }
}

impl Stream for PointsWGPU {
    type EP = Self;
    type T = f32;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    #[inline]
    fn point(&mut self, p: &Coord<Self::T>, _z: Option<u8>) {
        self.v_buffer.push(Vertex {
            pos: [p.x, p.y, 0.],
        });
    }
}
