use num_traits::Float;
use std::cell::RefCell;
use std::rc::Rc;

use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;

pub struct TransformRadians<F> {
  stream: Rc<RefCell<Box<dyn TransformStream<F>>>>,
}

impl<F> TransformRadians<F>
where
  F: Float + 'static,
{
  pub fn new() -> StreamProcessor<F> {
    return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream<F>>>>| {
      return Rc::new(RefCell::new(Box::new(Self { stream })));
    });
  }
}

impl<F> TransformStream<F> for TransformRadians<F>
where
  F: Float,
{
  fn point(&mut self, x: F, y: F, m: Option<u8>) {
    let mut stream = self.stream.borrow_mut();
    stream.point(x.to_radians(), y.to_radians(), m);
  }
}
