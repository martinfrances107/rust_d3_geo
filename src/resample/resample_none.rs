use std::cell::RefCell;
use std::rc::Rc;

use num_traits::Float;
use num_traits::FloatConst;

use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
use crate::Transform;

// function resampleNone(project) {
//   return transformer({
//     point: function(x, y) {
//       x = project(x, y);
//       this.stream.point(x[0], x[1]);
//     }
//   });
// }

pub struct ResampleNone<F>
where
  F: Float + 'static,
{
  project: Rc<RefCell<Box<dyn Transform<F>>>>,
  stream: Rc<RefCell<Box<dyn TransformStream<F>>>>,
}

impl<F> ResampleNone<F>
where
  F: Float + FloatConst + 'static,
{
  pub fn new(project: Rc<RefCell<Box<dyn Transform<F>>>>) -> StreamProcessor<F> {
    return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream<F>>>>| {
      return Rc::new(RefCell::new(Box::new(Self {
        project: project.clone(),
        stream,
      })));
    });
  }
}

impl<F> TransformStream<F> for ResampleNone<F>
where
  F: Float,
{
  fn point(&mut self, x: F, y: F, m: Option<u8>) {
    let mut stream = self.stream.borrow_mut();
    let project = &*self.project.borrow();
    let p = project.transform(&[x, y]);
    stream.point(p[0], p[1], m);
  }
}
