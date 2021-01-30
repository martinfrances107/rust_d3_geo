use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

use crate::stream::Stream;
use crate::stream::StreamNode;
use crate::transform_stream::StreamProcessor;
use crate::Transform;

pub struct TransformRotate<T> {
    rotate: Rc<Box<dyn Transform<T>>>,
    stream: StreamNode<T>,
}

impl<T: CoordFloat + FloatConst + 'static> TransformRotate<T> {
    #[inline]
    pub fn new(rotate: Rc<Box<dyn Transform<T>>>) -> StreamProcessor<T> {
        Box::new(move |stream: StreamNode<T>| {
            Rc::new(RefCell::new(Box::new(Self {
                rotate: rotate.clone(),
                stream,
            })))
        })
    }
}

impl<T: CoordFloat + FloatConst> Stream<T> for TransformRotate<T> {
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        let rotate = self.rotate.clone();
        let r = rotate.transform(&Coordinate { x, y });
        // Warning the javascript version return the value below but I thnk it break the implied spec!!!!
        stream.point(r.x, r.y, m);
    }
}
