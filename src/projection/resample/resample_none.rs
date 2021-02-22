use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::{cell::RefCell, rc::Rc};

use crate::stream::StreamPostClipNode;
use crate::stream::StreamResampleTrait;
use crate::stream::{Stream, StreamResampleNode};
use crate::stream::{StreamNodeStub, StreamSimpleNode};
use crate::Transform;

pub struct ResampleNone<T>
where
    T: CoordFloat,
{
    project: Rc<Box<dyn Transform<C = Coordinate<T>, TcC = Coordinate<T>>>>,
    stream: StreamSimpleNode<T>,
}

impl<T: CoordFloat + FloatConst + Default + 'static> ResampleNone<T> {
    #[inline]
    pub fn gen_node(
        project: Rc<Box<dyn Transform<C = Coordinate<T>, TcC = Coordinate<T>>>>,
    ) -> StreamResampleNode<T> {
        Rc::new(RefCell::new(Self {
            project: project.clone(),
            stream: StreamNodeStub::new(),
        }))
    }
}

impl<T: CoordFloat + FloatConst> StreamResampleTrait<T> for ResampleNone<T> {
    fn stream_postclip_in(&mut self, _stream_in: StreamPostClipNode<T>) {}
}
impl<T: CoordFloat + FloatConst> Stream for ResampleNone<T> {
    type C = Coordinate<T>;
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        let mut s = self.stream.borrow_mut();
        let project = &*self.project;
        let t = project.transform(&p);
        s.point(t, m);
    }
}
