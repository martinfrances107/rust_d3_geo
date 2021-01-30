use geo::CoordFloat;
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

use crate::stream::Stream;
use crate::stream::StreamNode;
use crate::transform_stream::StreamProcessor;

pub struct TransformRadians<T> {
    stream: StreamNode<T>,
}

impl<T: CoordFloat + FloatConst + 'static> TransformRadians<T> {
    #[inline]
    pub fn new() -> StreamProcessor<T> {
        Box::new(move |stream: StreamNode<T>| Rc::new(RefCell::new(Box::new(Self { stream }))))
    }
}

impl<T: CoordFloat + FloatConst> Stream<T> for TransformRadians<T> {
    #[inline]
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let mut s = self.stream.borrow_mut();
        s.point(x.to_radians(), y.to_radians(), m);
    }
}
