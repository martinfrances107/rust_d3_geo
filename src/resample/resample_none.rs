use std::cell::RefCell;
use std::rc::Rc;

use num_traits::Float;
use num_traits::FloatConst;

use crate::transform_stream::TransformStream;
use crate::Transform;
// use crate::transform_stream::TransformStreamIdentity;

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
  stream: Option<Rc<RefCell<Box<dyn TransformStream<F>>>>>,
}

impl<F> ResampleNone<F>
where
  F: Float + FloatConst,
{
  pub fn new(project: Rc<RefCell<Box<dyn Transform<F>>>>) -> Self {
    return ResampleNone {
      project: project.clone(),
      stream: None,
    };
  }
}

impl<F> TransformStream<F> for ResampleNone<F>
where
  F: Float,
{
  // fn stream(&mut self, _stream: &Box<dyn TransformStream<F>>) {}
  fn stream(&mut self, stream: &Rc<RefCell<Box<dyn TransformStream<F>>>>) {
    self.stream = Some(stream.clone());
  }

  fn point(&mut self, x: F, y: F, z: Option<F>) {
    match &self.stream {
      Some(s) => {
        let mut stream = s.borrow_mut();
        let project = &*self.project.borrow();
        let p = project.transform(&[x, y]);
        stream.point(p[0], p[1], None);
      }
      None => {}
    }
  }
}
