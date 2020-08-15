use num_traits::Float;
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

// Define the default implementation of the trait.
// use crate::stream::GeoStream;

pub type StreamProcessor<F> = Box<dyn Fn(Rc<RefCell<Box<dyn TransformStream<F>>>>) -> Rc<RefCell<Box<dyn TransformStream<F>>>>>;
pub struct StreamProcessorIdentity {}

impl StreamProcessorIdentity {
  pub fn new<F>(
  ) -> StreamProcessor<F>
  where
    F: Float + 'static,
  {
    return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream<F>>>>| {
      return stream;
    });
  }
}

impl<F> TransformStream<F> for StreamProcessorIdentity where F: Float + FloatConst + 'static {}

pub struct TransformStreamIdentity {}
impl<F> TransformStream<F> for TransformStreamIdentity where F: Float + FloatConst + 'static {}

/// Define the default implementation of the trait.
pub trait TransformStream<F>
where
  F: Float,
{
  fn point(&mut self, _x: F, _y: F, _z: Option<F>) {}
  fn sphere(&mut self) {}
  fn line_start(&mut self) {}
  fn line_end(&mut self) {}
  fn polygon_start(&mut self) {}
  fn polygon_end(&mut self) {}
}
