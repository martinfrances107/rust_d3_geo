use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use std::fmt::Display;
use std::ops::AddAssign;

use num_traits::FloatConst;

use crate::Transform;

#[derive(Clone, Debug)]
pub struct Compose<T, TA, TB>
where
    T: CoordFloat + FloatConst,
    TA: Clone + Transform<C = Coordinate<T>>,
    TB: Clone + Transform<C = Coordinate<T>>,
{
    pub a: TA,
    pub b: TB,
}

impl<T, TA, TB> Compose<T, TA, TB>
where
    T: CoordFloat + FloatConst,
    TA: Clone + Transform<C = Coordinate<T>>,
    TB: Clone + Transform<C = Coordinate<T>>,
{
    #[inline]
    pub fn new(a: TA, b: TB) -> Compose<T, TA, TB> {
        Compose { a, b }
    }
}

impl<T, TA, TB> Transform for Compose<T, TA, TB>
where
    TA: Clone + Transform<C = Coordinate<T>>,
    TB: Clone + Transform<C = Coordinate<T>>,
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
    // Apply A then B.
    fn transform(&self, coordinates: &<TA as Transform>::C) -> Coordinate<T> {
        let temp = self.a.transform(coordinates);
        self.b.transform(&temp)
    }

    // Apply B them A.
    fn invert(&self, coordinates: &<TA as Transform>::C) -> Coordinate<T> {
        let temp = self.b.invert(coordinates);
        self.a.invert(&temp)
    }
}

impl<SINK, T, TA, TB> Stream for StreamNode<Compose<T, TA, TB>, SINK, T>
where
    SINK: Stream<SC = Coordinate<T>>,
    TA: Clone + Transform<C = Coordinate<T>>,
    TB: Clone + Transform<C = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;
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
