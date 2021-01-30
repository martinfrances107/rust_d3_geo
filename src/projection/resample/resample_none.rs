use crate::stream::StreamNode;
use crate::Transform;
use crate::{stream::Stream, transform_stream::StreamProcessor};
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::{cell::RefCell, rc::Rc};

pub struct ResampleNone<T> {
    project: Rc<Box<dyn Transform<T>>>,
    stream: StreamNode<T>,
}

// pub fn new(project: Rc<Box<dyn Transform<T>>>, delta2: T) -> StreamProcessor<T> {
//     return Box::new(move |stream: StreamNode| {
//         return Rc::new(RefCell::new(Box::new(Self {

// pub type StreamProcessor<T> = Box<dyn Fn(StreamNode) -> StreamNode>;
impl<T: CoordFloat + FloatConst + 'static> ResampleNone<T> {
    #[inline]
    pub fn new(project: Rc<Box<dyn Transform<T>>>) -> StreamProcessor<T> {
        Box::new(move |stream: StreamNode<T>| {
            Rc::new(RefCell::new(Box::new(Self {
                project: project.clone(),
                stream,
            })))
        })
    }
}

impl<T: CoordFloat + FloatConst> Stream<T> for ResampleNone<T> {
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let mut s = self.stream.borrow_mut();
        let project = &*self.project;
        let p = project.transform(&Coordinate { x, y });
        s.point(p.x, p.y, m);
    }
}
