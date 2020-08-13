// use num_traits::cast::FromPrimitive;
use num_traits::Float;
use std::cell::RefCell;
use std::rc::Rc;

use crate::transform_stream::TransformStream;
use crate::Transform;

pub struct TransformRotate<F> {
  stream: Option<Rc<RefCell<Box<dyn TransformStream<F>>>>>,
  rotate: Box<dyn Transform<F>>,
}

impl<F> TransformStream<F> for TransformRotate<F>
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
        let r = self.rotate.transform(&[x, y]);
        // Warning the javascript version return the value below but I thnk it break the implied spec!!!!
        stream.point(r[0], r[1], z);
      }
      None => {}
    }
  }
}
