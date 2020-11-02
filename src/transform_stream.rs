use delaunator::Point;
use std::cell::RefCell;
use std::rc::Rc;

// Define the default implementation of the trait.
// use crate::stream::GeoStream;

pub type StreamProcessor =
    Box<dyn Fn(Rc<RefCell<Box<dyn TransformStream>>>) -> Rc<RefCell<Box<dyn TransformStream>>>>;
pub struct StreamProcessorIdentity {}

impl StreamProcessorIdentity {
    pub fn new() -> StreamProcessor {
        return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream>>>| {
            return stream;
        });
    }
}

impl TransformStream for StreamProcessorIdentity {}

pub struct TransformStreamIdentity {}
impl TransformStream for TransformStreamIdentity {}

/// Define the default implementation of the trait.
pub trait TransformStream {
    fn point(&mut self, _x: f64, _y: f64, _message: Option<u8>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}
