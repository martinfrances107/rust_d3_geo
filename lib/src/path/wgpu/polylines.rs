use core::hash::Hash;
use core::hash::Hasher;
use core::mem;
use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

use bytemuck::Pod;
use bytemuck::Zeroable;
use geo_types::Coord;

use crate::stream::Stream;

use crate::path::Result;

use super::Vertex;

/// Primitive Restart
///
/// "which allows an indexed draw command to end the current primitive when a
/// specified index is processed, beginning a new one of the same type with the next index."
///
/// <https://www.supergoodcode.com/restart/>
pub static PRIMITVE_RESTART_TOKEN: Index = Index(u32::MAX);

/// Make Coord Hashable
/// treat coord as byes for hashing
#[derive(Copy, Clone, Debug, PartialEq)]
struct CoordHashable(Coord<f32>);

impl Eq for CoordHashable {}

impl Hash for CoordHashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x.to_bits().hash(state);
        self.0.y.to_bits().hash(state);
    }
}

/// Elements of the index buffer.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Pod, Zeroable)]
pub struct Index(u32);

impl Index {
    /// description a `wgpu::VertexState parameter`
    /// The layout in memory of the index buffer.
    #[must_use]
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Index>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32,
            }],
        }
    }
}

/// Stream path endpoint: Used when rendering to a HTML Canvas element.
///
/// Wraps a Path2d object, and implements STREAM trait.
#[derive(Clone, Debug, PartialEq)]
pub struct PolyLines {
    /// points in a form ready to be shipped to the GPU.
    pub vertex_buffer: Vec<Vertex>,
    /// index_buffer is a form ready to be shipped to the GPU.
    pub index_buffer: Vec<Index>,
    /// Tracks if a point has been seen before
    index_store: HashMap<CoordHashable, usize>,

    // Increment when adding a new point the vertex_buffer
    next_index: usize,
}

impl Default for PolyLines {
    #[inline]
    fn default() -> Self {
        // A default capacity 200 points!!
        // TODO consider a new(capacity: usize)
        // which allows this to be overrriden.
        let capacity = 200usize;
        Self {
            vertex_buffer: Vec::with_capacity(capacity),
            index_buffer: Vec::with_capacity(capacity),
            index_store: HashMap::with_capacity(capacity),
            next_index: 0usize,
        }
    }
}

/// Return path2d, blanking the stored value.
///
/// Architecture Discussion:
///
/// I am making the assumption here that in a animation frame
/// repeated calls to [PolyLines::result] will return approximatly
/// the same number of elements.
impl Result for PolyLines {
    type Out = (Vec<Vertex>, Vec<Index>);
    #[inline]
    fn result(&mut self) -> Self::Out {
        let mut v_out = Vec::with_capacity(self.vertex_buffer.capacity());
        mem::swap(&mut v_out, &mut self.vertex_buffer);
        let mut i_out = Vec::with_capacity(self.index_buffer.capacity());
        mem::swap(&mut i_out, &mut self.index_buffer);
        self.next_index = 0;
        self.index_store.clear();
        (v_out, i_out)
    }
}

impl Stream for PolyLines {
    type EP = Self;
    type T = f32;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    #[inline]
    fn point(&mut self, p: &Coord<Self::T>, _z: Option<u8>) {
        // Check the points store to see if this point has a index.
        let p_key = CoordHashable(*p);
        match self.index_store.entry(p_key) {
            Occupied(o) => {
                // Point has been seen before just update the index list.
                let index = o.get();
                self.index_buffer.push(Index(*index as u32))
            }
            Vacant(v) => {
                let index = v.insert(self.next_index);
                self.index_buffer.push(Index(*index as u32));
                self.vertex_buffer.push(Vertex { pos: [p.x, p.y] });
                self.next_index += 1;
            }
        };
    }

    fn line_end(&mut self) {
        // Let the GPU know that a new line_strip is about to start.
        self.index_buffer.push(PRIMITVE_RESTART_TOKEN.clone());
    }
}
