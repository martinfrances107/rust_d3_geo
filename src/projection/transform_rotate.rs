use delaunator::Point;
use std::cell::RefCell;
use std::rc::Rc;

use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
use crate::Transform;

pub struct TransformRotate {
    rotate: Rc<RefCell<Box<dyn Transform>>>,
    stream: Rc<RefCell<Box<dyn TransformStream>>>,
}

impl TransformRotate {
    pub fn new(rotate: Rc<RefCell<Box<dyn Transform>>>) -> StreamProcessor {
        return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream>>>| {
            return Rc::new(RefCell::new(Box::new(Self {
                rotate: rotate.clone(),
                stream,
            })));
        });
    }
}

impl TransformStream for TransformRotate {
    fn point(&mut self, x: f64, y: f64, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        let rotate = self.rotate.borrow();
        let r = rotate.transform(&Point { x, y });
        // Warning the javascript version return the value below but I thnk it break the implied spec!!!!
        stream.point(r.x, r.y, m);
    }
}
