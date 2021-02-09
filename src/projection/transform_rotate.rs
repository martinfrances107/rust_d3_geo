use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

use crate::stream::Stream;
use crate::stream::StreamInTrait;
use crate::stream::StreamNodeStub;
use crate::stream::StreamSimple;
use crate::stream::StreamSimpleNode;

use crate::Transform;

pub struct TransformNode<T> {
    transform: Rc<Box<dyn Transform<T>>>,
    stream: StreamSimpleNode<T>,
}

impl<T: CoordFloat + FloatConst + 'static> StreamInTrait<T> for TransformNode<T> {
    #[inline]
    fn stream_in(&mut self, stream: StreamSimpleNode<T>) {
        self.stream = stream;
    }
}

impl<T: CoordFloat + FloatConst + 'static> TransformNode<T> {
    #[inline]
    pub fn gen_node(rotate: Rc<Box<dyn Transform<T>>>) -> StreamSimpleNode<T> {
        {
            Rc::new(RefCell::new(Box::new(Self {
                transform: rotate.clone(),
                stream: StreamNodeStub::new(),
            })))
        }
    }
}

impl<T> StreamSimple<T> for TransformNode<T> where T: CoordFloat + FloatConst + 'static {}

impl<T: CoordFloat + FloatConst> Stream<T> for TransformNode<T> {
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        let rotate = self.transform.clone();
        let r = rotate.transform(&p);
        // Warning the javascript version return the value below but I think it break the implied spec!!!!
        stream.point(r, m);
    }
}
