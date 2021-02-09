// use super::StreamTransformTrait;
use crate::stream::StreamInTrait;
use crate::stream::StreamSimple;
use crate::stream::StreamSimpleNode;
use crate::stream::{Stream, StreamSimpleNodeStub};
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TransformRadians<T> {
    stream: StreamSimpleNode<T>,
}

impl<T: CoordFloat + FloatConst + 'static> TransformRadians<T> {
    #[inline]
    pub fn gen_node() -> StreamSimpleNode<T> {
        Rc::new(RefCell::new(Box::new(Self {
            stream: StreamSimpleNodeStub::new(),
        })))
    }
}

impl<T> StreamSimple<T> for TransformRadians<T> where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for TransformRadians<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, sink: StreamSimpleNode<T>) {}
}

impl<T: CoordFloat + FloatConst> Stream<T> for TransformRadians<T> {
    #[inline]
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        let mut s = self.stream.borrow_mut();
        s.point(
            Coordinate {
                x: p.x.to_radians(),
                y: p.y.to_radians(),
            },
            m,
        );
    }
}
