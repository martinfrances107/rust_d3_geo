// use num_traits::cast::FromPrimitive;
use num_traits::Float;
use std::cell::RefCell;
use std::rc::Rc;

use crate::transform_stream::TransformStream;

pub struct TransformRadians<F> {
  stream: Option<Rc<RefCell<Box<dyn TransformStream<F>>>>>,
}

impl<F> TransformStream<F> for TransformRadians<F>
where
  F: Float,
{
  fn stream(&mut self, stream: &Rc<RefCell<Box<dyn TransformStream<F>>>>) {
    self.stream = Some(stream.clone());
  }

  fn point(&mut self, x: F, y: F, z: Option<F>) {
    match &self.stream {
      Some(s) => {
        let mut stream = s.borrow_mut();
        stream.point(x.to_radians(), y.to_radians(), z);
      }
      None => {}
    }
  }
}
