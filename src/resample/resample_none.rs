use std::cell::RefCell;
use std::rc::Rc;

use delaunator::Point;

use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
use crate::Transform;

// function resampleNone(project) {
//   return transformer({
//     point: f64unction(x, y) {
//       x = project(x, y);
//       this.stream.point(x[0], x[1]);
//     }
//   });
// }

pub struct ResampleNone
{
  project: Rc<RefCell<Box<dyn Transform>>>,
  stream: Rc<RefCell<Box<dyn TransformStream>>>,
}

impl ResampleNone
{
  pub fn new(project: Rc<RefCell<Box<dyn Transform>>>) -> StreamProcessor {
    return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream>>>| {
      return Rc::new(RefCell::new(Box::new(Self {
        project: project.clone(),
        stream,
      })));
    });
  }
}

impl TransformStream for ResampleNone
{
  fn point(&mut self, x: f64, y: f64, m: Option<u8>) {
    let mut stream = self.stream.borrow_mut();
    let project = &*self.project.borrow();
    let p = project.transform(&Point{x, y});
    stream.point(p.x, p.y, m);
  }
}
