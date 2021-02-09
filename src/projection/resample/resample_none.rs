use crate::stream::Stream;
use crate::stream::StreamInTrait;
use crate::stream::StreamSimple;
use crate::stream::{StreamNodeStub, StreamSimpleNode};
use crate::Transform;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::{cell::RefCell, rc::Rc};

pub struct ResampleNone<T> {
    project: Rc<Box<dyn Transform<T>>>,
    stream: StreamSimpleNode<T>,
}

impl<T: CoordFloat + FloatConst + 'static> ResampleNone<T> {
    #[inline]
    pub fn gen_node(project: Rc<Box<dyn Transform<T>>>) -> StreamSimpleNode<T> {
        Rc::new(RefCell::new(Box::new(Self {
            project: project.clone(),
            stream: StreamNodeStub::new(),
        })))
    }
}

impl<T> StreamSimple<T> for ResampleNone<T> where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for ResampleNone<T> where T: CoordFloat + FloatConst {}

impl<T: CoordFloat + FloatConst> Stream<T> for ResampleNone<T> {
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        let mut s = self.stream.borrow_mut();
        let project = &*self.project;
        let t = project.transform(&p);
        s.point(t, m);
    }
}
