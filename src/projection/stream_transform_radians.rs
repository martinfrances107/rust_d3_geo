use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

use crate::stream::Stream;
use crate::stream::StreamTransformNode;
use crate::stream::StreamTransformNodeStub;

pub trait StreamTransformIn<T>
where
    T: CoordFloat,
{
    fn stream_transform_in(&mut self, _stream: StreamTransformNode<T>) {
        panic!("Must be overriden.");
    }
}

#[derive(Clone, Default)]
pub struct StreamTransformRadiansNodeStub;
impl StreamTransformRadiansNodeStub {
    #[inline]
    pub fn new<T>() -> StreamTransformRadiansNode<T>
    where
        T: CoordFloat + FloatConst + std::default::Default + 'static,
    {
        Rc::new(RefCell::new(Box::new(StreamTransformRadians {
            stream: StreamTransformNodeStub::new(),
        })))
    }
}
pub type StreamTransformRadiansNode<T> = Rc<RefCell<Box<StreamTransformRadians<T>>>>;
impl<T> Stream<T> for StreamTransformRadiansNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamTransformIn<T> for StreamTransformRadiansNode<T> where T: CoordFloat {}

pub struct StreamTransformRadians<T: CoordFloat> {
    stream: StreamTransformNode<T>,
}

impl<T: CoordFloat + FloatConst + std::default::Default + 'static> StreamTransformRadians<T> {
    #[inline]
    pub fn gen_node() -> StreamTransformRadiansNode<T> {
        Rc::new(RefCell::new(Box::new(Self {
            stream: StreamTransformNodeStub::new(),
        })))
    }
}

impl<T> StreamTransformIn<T> for StreamTransformRadians<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn stream_transform_in(&mut self, stream: StreamTransformNode<T>) {
        self.stream = stream;
    }
}

impl<T: CoordFloat + FloatConst> Stream<T> for StreamTransformRadians<T> {
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
