// use num_traits::cast::FromPrimitive;
use num_traits::Float;

use crate::transform_stream::TransformStream;
use crate::Transform;

pub struct TransformRotate<F>{
  stream: Box<dyn TransformStream<F>>,
  rotate: Box<dyn Transform<F>>,
 }

impl <F>TransformStream<F> for TransformRotate<F>
where F: Float {
  fn stream(&mut self, stream: Box<dyn TransformStream<F>>) {
    self.stream = stream;
  }

  fn point(&mut self, x: F, y: F, z: Option<F>) {
    let r = self.rotate.transform(&[x, y]);
    // Warning the javascript version return the value below but I thnk it break the implied spec!!!!
    self.stream.point(r[0], r[1], z);
  }
}