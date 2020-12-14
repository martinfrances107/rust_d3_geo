use geo::Point;
use num_traits::Float;
use std::cell::RefCell;
use std::rc::Rc;

use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
use crate::Transform;

pub struct TransformRotate<T> {
    rotate: Rc<RefCell<Box<dyn Transform<T>>>>,
    stream: Rc<RefCell<Box<dyn TransformStream<T>>>>,
}

impl<T: Float + 'static> TransformRotate<T> {
    pub fn new(rotate: Rc<RefCell<Box<dyn Transform<T>>>>) -> StreamProcessor<T> {
        return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream<T>>>>| {
            return Rc::new(RefCell::new(Box::new(Self {
                rotate: rotate.clone(),
                stream,
            })));
        });
    }
}

impl<T: Float> TransformStream<T> for TransformRotate<T> {
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        let rotate = self.rotate.borrow();
        let r = rotate.transform(&Point::new(x, y));
        // Warning the javascript version return the value below but I thnk it break the implied spec!!!!
        stream.point(r.x(), r.y(), m);
    }
}
