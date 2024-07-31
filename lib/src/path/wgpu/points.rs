use core::mem;

use geo_types::Coord;

use crate::stream::Stream;

// use super::PointRadiusTrait;
use crate::path::Result;

use super::Vertex;

/// Stream path endpoint: Used when rendering to a HTML Canvas element.
///
/// Wraps a Path2d object, and implements STREAM trait.
#[derive(Clone, Debug, PartialEq)]
pub struct Points {
    /// Vertext buffer is a form ready to be shipped to the GPU.
    pub v_buffer: Vec<Vertex>,
}

impl Default for Points {
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
impl Result for Points {
    type Out = Vec<Vertex>;
    #[inline]
    fn result(&mut self) -> Self::Out {
        let mut out = Vec::with_capacity(self.v_buffer.capacity());
        mem::swap(&mut out, &mut self.v_buffer);
        out
    }
}

impl Stream for Points {
    type EP = Self;
    type T = f32;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    #[inline]
    fn point(&mut self, p: &Coord<Self::T>, _z: Option<u8>) {
        self.v_buffer.push(Vertex {
            pos: [p.x, p.y],
        });
    }
}
