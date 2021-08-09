use crate::projection::StreamNode;
use std::fmt::Display;
use std::ops::AddAssign;
// use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use super::stream_transform::StreamTransform;
// use crate::stream::stream_in_trait::StreamIn;
// use crate::projection::ProjectionRawTrait;
use crate::stream::Stream;
// use crate::Transform;

#[derive(Clone, Default, Debug)]
pub struct StreamTransformRadians {}

impl<T, SINK> Stream for StreamNode<StreamTransformRadians, SINK, T>
where
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.sink.borrow_mut().point(
            &Coordinate {
                x: p.x.to_radians(),
                y: p.y.to_radians(),
            },
            m,
        );
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
