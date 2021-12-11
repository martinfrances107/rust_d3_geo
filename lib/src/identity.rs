use std::fmt::Debug;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

#[derive(Clone, Debug)]
pub struct Identity {}

impl<EP, SINK, T> Stream for StreamNode<EP, Identity, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        self.sink.get_endpoint()
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<Self::T>, m: Option<u8>) {
        self.sink.point(p, m)
    }

    #[inline]
    fn sphere(&mut self) {
        self.sink.sphere()
    }

    #[inline]
    fn line_start(&mut self) {
        self.sink.line_start();
    }

    #[inline]
    fn line_end(&mut self) {
        self.sink.line_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.sink.polygon_start();
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.sink.polygon_end();
    }
}
