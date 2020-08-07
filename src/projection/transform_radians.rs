// use num_traits::cast::FromPrimitive;
use num_traits::Float;

use crate::transform_stream::TransformStream;

pub struct TransformRadians<F>{
  stream: Box<dyn TransformStream<F>>,
 }

impl <F>TransformStream<F> for TransformRadians<F>
where F: Float {
  fn stream(&mut self, stream: Box<dyn TransformStream<F>>) {
    self.stream = stream;
  }

  fn point(&mut self, x: F, y: F, z: Option<F>) {
    self.stream.point(x.to_radians(), y.to_radians(),z);
  }
}