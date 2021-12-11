use std::fmt::Debug;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::StreamNode;

/// TODO: Can this be optimised away?
#[derive(Clone, Default, Debug)]
pub struct StreamTransformRadians {}

impl<EP, T, SINK> Stream for StreamNode<EP, StreamTransformRadians, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        self.sink.get_endpoint()
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.sink.point(
            &Coordinate {
                x: p.x.to_radians(),
                y: p.y.to_radians(),
            },
            m,
        );
    }

    #[inline]
    fn sphere(&mut self) {
        self.sink.sphere();
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
        self.sink.polygon_start()
    }
    #[inline]
    fn polygon_end(&mut self) {
        self.sink.polygon_end();
    }
}
