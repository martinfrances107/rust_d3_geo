use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

#[derive(Clone, Debug)]
pub struct Identity {}

impl<SINK, T> Stream for StreamNode<Identity, SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn point(&mut self, p: &Coordinate<Self::T>, m: Option<u8>) {
        self.sink.borrow_mut().point(p, m)
    }

    #[inline]
    fn sphere(&mut self) {
        self.sink.borrow_mut().sphere()
    }

    #[inline]
    fn line_start(&mut self) {
        self.sink.borrow_mut().line_start();
    }

    #[inline]
    fn line_end(&mut self) {
        self.sink.borrow_mut().line_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.sink.borrow_mut().polygon_start();
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.sink.borrow_mut().polygon_end();
    }
}
