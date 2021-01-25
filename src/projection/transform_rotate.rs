use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

use crate::stream::Stream;
use crate::Transform;

pub struct TransformRotate<T> {
    rotate: Rc<RefCell<Box<dyn Transform<T>>>>,
    stream: Rc<RefCell<Box<dyn Stream<T>>>>,
}

impl<T: CoordFloat + FloatConst + 'static> TransformRotate<T> {
    pub fn new(
        rotate: Rc<RefCell<Box<dyn Transform<T>>>>,
    ) -> Box<dyn Fn(Rc<RefCell<Box<dyn Stream<T>>>>) -> Box<dyn Stream<T>>> {
        return Box::new(move |stream: Rc<RefCell<Box<dyn Stream<T>>>>| {
            return Box::new(Self {
                rotate: rotate.clone(),
                stream,
            });
        });
    }
}

impl<T: CoordFloat + FloatConst> Stream<T> for TransformRotate<T> {
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        let rotate = self.rotate.borrow();
        let r = rotate.transform(&Coordinate { x, y });
        // Warning the javascript version return the value below but I thnk it break the implied spec!!!!
        stream.point(r.x, r.y, m);
    }
}
