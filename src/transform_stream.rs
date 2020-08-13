use num_traits::Float;
use num_traits::FloatConst;
use std::cell::{RefCell};
use std::rc::Rc;

// Define the default implementation of the trait.
// use crate::stream::GeoStream;

#[derive(Debug)]
pub struct TransformStreamIdentity {
// where F: Float {
  // phantom: PhantomData<F>,
}

impl TransformStreamIdentity {
  pub fn new<F>() -> impl TransformStream<F>
  where F: Float + FloatConst + 'static {
    return TransformStreamIdentity{
      // phantom: PhantomData
    };
  }
}

impl<F> TransformStream<F> for TransformStreamIdentity where F: Float + FloatConst + 'static{}

/// Define the default implementation of the trait.
pub trait TransformStream<F>
where
  F: Float,
{
  fn stream(&mut self, _stream: &Rc<RefCell<Box<dyn TransformStream<F>>>>) {}
  fn point(&mut self, _x: F, _y: F, _z: Option<F>) {}
  fn sphere(&mut self) {}
  fn line_start(&mut self) {}
  fn line_end(&mut self) {}
  fn polygon_start(&mut self) {}
  fn polygon_end(&mut self) {}
}
