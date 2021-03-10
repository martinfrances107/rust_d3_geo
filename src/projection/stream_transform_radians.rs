use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::marker::PhantomData;

use crate::projection::stream_transform::StreamTransform;
// use crate::stream::stream_dummy::StreamDummy;
use crate::stream::Stream;
use crate::stream::StreamClone;
// use crate::stream::StreamTransformNode;
// use crate::stream::stream_transform_node_stub::StreamTransformNodeStub;

pub trait StreamTransformIn<T>
where
    T: CoordFloat + FloatConst + Default,
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
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(self.clone())
    }
}

impl<T> Stream for StreamTransformRadiansNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type C = Coordinate<T>;
}
impl<T> StreamTransformIn<T> for Box<StreamTransformRadians<T>> where
    T: CoordFloat + FloatConst + Default
{
}

pub struct StreamTransformRadians<T: CoordFloat + FloatConst + Default + 'static> {
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
    T: CoordFloat + FloatConst + Default,
{
    #[inline]
    fn stream_transform_in(&mut self, stream: StreamTransform<T>) {
        self.stream = stream;
    }
}

// impl<T: CoordFloat + FloatConst + Default + 'static> StreamClone for StreamTransformRadians<T> {
//     type RetType = Box<dyn Stream<C = Coordinate<T>>>;
//     #[inline]
//     fn box_clone(&self) -> Self::RetType {
//         Box::new(Self {
//             stream: self.stream.clone(),
//         })
//     }
// }

impl<T: CoordFloat + FloatConst + Default + 'static> Stream for StreamTransformRadians<T> {
    type C = Coordinate<T>;
    #[inline]
    fn point(&mut self, p: Self::C, m: Option<u8>) {
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
