use std::cell::RefCell;
use std::rc::Rc;

use geo::Coordinate;
use num_traits::Float;

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

pub struct ResampleNone<T> {
    project: Rc<RefCell<Box<dyn Transform<T>>>>,
    stream: Rc<RefCell<Box<dyn TransformStream<T>>>>,
}

impl<T: Float + 'static> ResampleNone<T> {
    pub fn new(project: Rc<RefCell<Box<dyn Transform<T>>>>) -> StreamProcessor<T> {
        return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream<T>>>>| {
            return Rc::new(RefCell::new(Box::new(Self {
                project: project.clone(),
                stream,
            })));
        });
    }
}

impl<T: Float> TransformStream<T> for ResampleNone<T> {
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        let project = &*self.project.borrow();
        let p = project.transform(&Coordinate { x, y });
        stream.point(p.x, p.y, m);
    }
}
