use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::marker::PhantomData;

use crate::stream::Stream;

use super::stream_transform::StreamTransform;

pub trait StreamTransformIn<T>
where
    T: CoordFloat + FloatConst + Default,
{
    fn stream_transform_in(&mut self, stream: StreamTransform<T>);
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

impl<T> Stream for StreamTransformRadiansNodeStub<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}

pub struct StreamTransformRadians<T: CoordFloat + FloatConst + Default> {
    stream: StreamTransform<T>,
}

impl<T> Default for StreamTransformRadians<T>
where
    T: CoordFloat + FloatConst + Default,
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

impl<T: CoordFloat + FloatConst + Default> Stream for StreamTransformRadians<T> {
    type C = Coordinate<T>;
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
        self.polygon_end();
    }
}
