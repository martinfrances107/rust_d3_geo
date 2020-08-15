// use num_traits::cast::FromPrimitive;
use num_traits::Float;
use std::cell::RefCell;
use std::rc::Rc;

use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
use crate::Transform;

pub struct TransformRotate<F> {
  rotate: Rc<RefCell<Box<dyn Transform<F>>>>,
  stream: Rc<RefCell<Box<dyn TransformStream<F>>>>,
}

impl<F> TransformRotate<F> {
  pub fn new(rotate: Rc<RefCell<Box<dyn Transform<F>>>>) -> StreamProcessor<F>
  where
    F: Float + 'static,
  {
    return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream<F>>>>| {
      return Rc::new(RefCell::new(Box::new(Self {
        rotate: rotate.clone(),
        stream,
      })));
    });
  }
}

impl<F> TransformStream<F> for TransformRotate<F>
where
  F: Float,
{
  fn point(&mut self, x: F, y: F, z: Option<F>) {
    let mut stream = self.stream.borrow_mut();
    let rotate = self.rotate.borrow();
    let r = rotate.transform(&[x, y]);
    // Warning the javascript version return the value below but I thnk it break the implied spec!!!!
    stream.point(r[0], r[1], z);
  }
}
