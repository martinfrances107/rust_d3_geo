use core::mem;

use bytemuck::{Pod, Zeroable};
use geo::CoordFloat;
use geo_types::Coord;

use crate::stream::Stream;

use super::PointRadiusTrait;
use super::Result;

// TODO fix Zeroable and Pod issue.
// #[derive(Clone, Copy, Pod, Zeroable)]
/// Representation of a points, sendable to a GPU.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex<T>
where
    T: CoordFloat,
{
    pub pos: [T; 2],
    // _tex_coord: [f32; 2],
}

/// Stream path endpoint: Used when rendering to a HTML Canvas element.
///
/// Wraps a Path2d object, and implements STREAM trait.
#[derive(Clone, Debug, PartialEq)]
pub struct PointsWGPU<T>
where
    T: CoordFloat,
{
    /// Vertext buffer is a form ready to be shipped to the GPU.
    pub v_buffer: Vec<Vertex<T>>,
}

impl<T> Default for PointsWGPU<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self { v_buffer: vec![] }
    }
}

/// Return path2d, blanking the stored value.
///
/// Architecture Discussion:
///
/// I am making the assumption here that in a animation frame
/// repeated calls to .result() will return approximatly
/// the same number of elements.
impl<T> Result for PointsWGPU<T>
where
    T: CoordFloat,
{
    type Out = Vec<Vertex<T>>;
    #[inline]
    fn result(&mut self) -> Self::Out {
        let mut out = Vec::with_capacity(self.v_buffer.capacity());
        mem::swap(&mut out, &mut self.v_buffer);
        out
    }
}

impl<T> Stream for PointsWGPU<T>
where
    T: CoordFloat,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    #[inline]
    fn point(&mut self, p: &Coord<Self::T>, _z: Option<u8>) {
        self.v_buffer.push(Vertex { pos: [p.x, p.y] });
    }
}
