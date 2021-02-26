use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::marker::PhantomData;

use crate::projection::stream_transform::StreamTransform;
use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::stream::StreamDummy;
// use crate::stream::StreamTransformNode;
use crate::stream::StreamTransformNodeStub;

pub trait StreamTransformIn<T>
where
    T: CoordFloat + FloatConst,
{
    // fn stream_transform_in(&mut self, _stream: StreamTransformNode<T>) {
    fn stream_transform_in(&mut self, _stream: StreamTransform<T>) {
        panic!("Must be overriden.");
    }
}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamTransformRadiansNodeStub<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}
// #[derive(Clone, Default)]
// pub struct StreamTransformRadiansNodeStub;
// impl<T> StreamTransformRadiansNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     #[inline]
//     pub fn new() -> StreamTransformRadiansNode<T> {
//         Rc::new(RefCell::new(Box::new(StreamTransformRadians {
//             stream: StreamTransformNodeStub::new(),
//         })))
//     }
// }
// pub type StreamTransformRadiansNode<T> = Box<StreamTransformRadians<T>>;
impl<T> StreamClone for StreamTransformRadiansNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(self.clone())
    }
}

impl<T> Stream for StreamTransformRadiansNodeStub<T> where T: CoordFloat + FloatConst + 'static {}
impl<T> StreamTransformIn<T> for Box<StreamTransformRadians<T>> where T: CoordFloat + FloatConst {}

pub struct StreamTransformRadians<T: CoordFloat + FloatConst + 'static> {
    stream: StreamTransform<T>,
}

impl<T> Default for StreamTransformRadians<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn default() -> Self {
        Self {
            stream: StreamTransform::default(),
        }
    }
}

impl<T> StreamTransformIn<T> for StreamTransformRadians<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn stream_transform_in(&mut self, stream: StreamTransform<T>) {
        self.stream = stream;
    }
}

impl<T: CoordFloat + FloatConst + 'static> StreamClone for StreamTransformRadians<T> {
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(*self.clone())
    }
}
impl<T: CoordFloat + FloatConst + 'static> Stream for StreamTransformRadians<T> {
    #[inline]
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        // let mut s = self.stream.borrow_mut();
        self.stream.point(
            Coordinate {
                x: p.x.to_radians(),
                y: p.y.to_radians(),
            },
            m,
        );
    }
}
