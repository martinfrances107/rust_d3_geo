// use delaunator::Point;
use num_traits::Float;
use std::cell::RefCell;
use std::rc::Rc;

use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;

pub struct TransformRadians<T> {
    stream: Rc<RefCell<Box<dyn TransformStream<T>>>>,
}

impl<T: Float + 'static> TransformRadians<T> {
    pub fn new() -> StreamProcessor<T> {
        return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream<T>>>>| {
            return Rc::new(RefCell::new(Box::new(Self { stream })));
        });
    }
}

impl<T: Float> TransformStream<T> for TransformRadians<T> {
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        stream.point(x.to_radians(), y.to_radians(), m);
    }
}
