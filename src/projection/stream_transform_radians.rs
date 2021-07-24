use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;
// use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use super::stream_transform::StreamTransform;
// use crate::stream::stream_in_trait::StreamIn;
// use crate::projection::ProjectionRawTrait;
// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
// use crate::Transform;

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

// impl<T> Stream for StreamTransformRadiansNodeStub<T>
// where
//     T: CoordFloat +FloatConst,
// {
//     type C = Coordinate<T>;
// }

#[derive(Debug)]
pub struct StreamTransformRadians<
    // PR: Transform<C = Coordinate<T>>,
    // SD,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
> {
    stream: STREAM,
}

impl<STREAM, T> StreamTransformRadians<STREAM, T>
where
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn new(stream: STREAM) -> Self
// where
        // Rc<PR>: Transform<C = Coordinate<T>>,
        // PR: Transform<C = Coordinate<T>>,
    {
        Self { stream }
    }
}

// impl<STREAM, T> StreamIn for StreamTransformRadians<STREAM, T>
// where
//     // PR: Transform<C = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
//     STREAM: Stream<SC = Coordinate<T>> + Default,
// {
//     type SInput = STREAM;
//     #[inline]
//     fn stream_in(&mut self, stream: Self::SInput) {
//         self.stream = stream;
//     }
// }

impl<STREAM, T> Stream for StreamTransformRadians<STREAM, T>
where
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;
    // type ST = T;
    // type SD = SD;
    // #[inline]
    // fn get_dst(
    //     &self,
    // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
    // {
    //     self.stream.get_dst()
    // }
    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
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
