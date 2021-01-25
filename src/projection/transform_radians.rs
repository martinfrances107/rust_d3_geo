use geo::CoordFloat;
// use delaunator::Point;
use num_traits::FloatConst;

use crate::{stream::Stream, transform_stream::StreamProcessor};
// use crate::transform_stream::StreamProcessor;

pub struct TransformRadians<T> {
    stream: Box<dyn Stream<T>>,
}

impl<T: CoordFloat + FloatConst + 'static> TransformRadians<T> {
    #[inline]
    pub fn new() -> StreamProcessor<T> {
        return Box::new(move |stream: Box<dyn Stream<T>>| Box::new(Self { stream }));
    }
}

impl<T: CoordFloat + FloatConst> Stream<T> for TransformRadians<T> {
    #[inline]
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        self.stream.point(x.to_radians(), y.to_radians(), m);
    }
}
