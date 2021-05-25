use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::Transform;

use super::stream_transform::StreamTransform;

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

// impl<T> Stream<T> for StreamTransformRadiansNodeStub<T>
// where
//     T: CoordFloat + Default + FloatConst,
// {
//     type C = Coordinate<T>;
// }

#[derive(Clone, Debug)]
pub struct StreamTransformRadians<
    P: Clone,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
> {
    stream: StreamTransform<P, T>,
}

impl<P, T> Default for StreamTransformRadians<P, T>
where
    P: Clone + Default + Transform<TcC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    fn default() -> Self {
        Self {
            stream: StreamTransform::default(),
        }
    }
}

impl<P, T> StreamTransformRadians<P, T>
where
    P: Clone,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    pub fn stream_in(&mut self, stream: StreamTransform<P, T>) {
        self.stream = stream;
    }
}

impl<P, T> Stream<T> for StreamTransformRadians<P, T>
where
    P: Clone + Default + Transform<TcC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C = Coordinate<T>;

    #[inline]
    fn get_dst(&self) -> StreamDst<T> {
        self.stream.get_dst()
    }
    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        self.stream.point(
            &Coordinate {
                x: p.x.to_radians(),
                y: p.y.to_radians(),
            },
            m,
        );
    }
    #[inline]
    fn sphere(&mut self) {
        self.stream.sphere();
    }
    #[inline]
    fn line_start(&mut self) {
        self.stream.line_start();
    }
    #[inline]
    fn line_end(&mut self) {
        self.stream.line_end();
    }
    #[inline]
    fn polygon_start(&mut self) {
        self.stream.polygon_start()
    }
    #[inline]
    fn polygon_end(&mut self) {
        self.stream.polygon_end();
    }
}
