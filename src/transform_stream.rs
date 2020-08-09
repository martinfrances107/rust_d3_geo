use num_traits::Float;

// Define the default implementation of the trait.
// use crate::stream::GeoStream;

#[derive(Debug)]
pub struct TransformStreamIdentity {}

impl TransformStreamIdentity {
  pub fn new<F>() -> Self
  where F: Float {
    return TransformStreamIdentity{};
  }
}

impl<F> TransformStream<F> for TransformStreamIdentity where F: Float {}

/// Define the default implementation of the trait.
pub trait TransformStream<F>
where
  F: Float,
{
  fn stream(&mut self, _stream: Box<dyn TransformStream<F>>) {}
  fn point(&mut self, _x: F, _y: F, _z: Option<F>) {}
  fn sphere(&mut self) {}
  fn line_start(&mut self) {}
  fn line_end(&mut self) {}
  fn polygon_start(&mut self) {}
  fn polygon_end(&mut self) {}
}
