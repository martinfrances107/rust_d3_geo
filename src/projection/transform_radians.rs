// use delaunator::Point;
use std::cell::RefCell;
use std::rc::Rc;

use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;

pub struct TransformRadians {
    stream: Rc<RefCell<Box<dyn TransformStream>>>,
}

impl TransformRadians {
    pub fn new() -> StreamProcessor {
        return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream>>>| {
            return Rc::new(RefCell::new(Box::new(Self { stream })));
        });
    }
}

impl TransformStream for TransformRadians {
    fn point(&mut self, x: f64, y: f64, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        stream.point(x.to_radians(), y.to_radians(), m);
    }
}
