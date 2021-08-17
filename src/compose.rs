use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;
use crate::Transform;

#[derive(Clone, Debug)]
pub struct Compose<T, TA, TB>
where
    T: CoordFloat,
    TA: Transform<T = T>,
    TB: Transform<T = T>,
{
    pub a: TA,
    pub b: TB,
}

impl<T, TA, TB> Compose<T, TA, TB>
where
    T: CoordFloat,
    TA: Transform<T = T>,
    TB: Transform<T = T>,
{
    #[inline]
    pub fn new(a: TA, b: TB) -> Compose<T, TA, TB> {
        Compose { a, b }
    }
}

impl<T, TA, TB> Transform for Compose<T, TA, TB>
where
    TA: Transform<T = T>,
    TB: Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;
    // Apply A then B.
    fn transform(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.a.transform(coordinate);
        self.b.transform(&temp)
    }

    // Apply B them A.
    fn invert(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.b.invert(coordinate);
        self.a.invert(&temp)
    }
}

impl<SINK, T, TA, TB> Stream for StreamNode<Compose<T, TA, TB>, SINK, T>
where
    SINK: Stream<T = T>,
    TA: Transform<T = T>,
    TB: Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.sink.borrow_mut().point(&self.raw.transform(p), m);
    }
    #[inline]
    fn sphere(&mut self) {
        self.sink.borrow_mut().sphere();
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
        self.sink.borrow_mut().polygon_start()
    }
    #[inline]
    fn polygon_end(&mut self) {
        self.sink.borrow_mut().polygon_end();
    }
}
