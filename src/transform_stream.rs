use num_traits::Float;
use std::cell::RefCell;
use std::rc::Rc;

// Define the default implementation of the trait.
// use crate::stream::GeoStream;

pub type StreamProcessor<T> = Box<
    dyn Fn(Rc<RefCell<Box<dyn TransformStream<T>>>>) -> Rc<RefCell<Box<dyn TransformStream<T>>>>,
>;
pub struct StreamProcessorIdentity {}

impl StreamProcessorIdentity {
    pub fn new<T: Float>() -> StreamProcessor<T> {
        return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream<T>>>>| {
            return stream;
        });
    }
}

impl<T> TransformStream<T> for StreamProcessorIdentity {}

pub struct TransformStreamIdentity {}
impl<T> TransformStream<T> for TransformStreamIdentity {}

/// Define the default implementation of the trait.
pub trait TransformStream<T> {
    fn point(&mut self, _x: T, _y: T, _message: Option<u8>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}
