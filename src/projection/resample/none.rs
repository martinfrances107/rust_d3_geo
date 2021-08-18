use geo::{CoordFloat, Coordinate};

use crate::projection::stream_node::StreamNode;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::Transform;

/// Resample None.
#[derive(Clone, Copy, Debug)]
pub struct None<PR, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    T: CoordFloat,
{
    projection_raw: PR,
}

impl<PR, T> Default for None<PR, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            projection_raw: PR::default(),
        }
    }
}

impl<PR, T> None<PR, T>
where
    T: CoordFloat,
    PR: ProjectionRaw<T = T> + Transform<T = T>,
{
    pub fn new(projection_raw: PR) -> None<PR, T> {
        Self { projection_raw }
    }
}

impl<PR, SINK, T> Stream for StreamNode<None<PR, T>, SINK, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn sphere(&mut self) {
        self.sink.borrow_mut().sphere()
    }

    #[inline]
    fn line_start(&mut self) {
        self.sink.borrow_mut().line_start()
    }

    #[inline]
    fn line_end(&mut self) {
        self.sink.borrow_mut().line_end()
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.sink.borrow_mut().polygon_start()
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.sink.borrow_mut().polygon_end()
    }

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let t = &self.raw.projection_raw.transform(p);
        self.sink.borrow_mut().point(t, m);
    }
}
