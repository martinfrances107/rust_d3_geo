use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use crate::clip::clip_sink_enum::ClipSinkEnum;
// use crate::projection::ProjectionRawTrait;
// use crate::stream::stream_dst::StreamDst;
use super::ResampleTrait;
use crate::stream::stream_in_trait::StreamIn;
use crate::stream::Stream;
use crate::Transform;

// #[derive(Debug)]
pub struct ResampleNone<'a, PR, STREAM, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
    STREAM: Stream<SC = Coordinate<T>>,
{
    pd: PhantomData<&'a u8>,
    projection_raw: &'a PR,
    /// Box to prevent infinite recusion.
    // pub stream: Box<ClipSinkEnum<'a, PR, T>>,
    pub stream: Box<STREAM>,
}

impl<'a, PR, STREAM, T> ResampleNone<'a, PR, STREAM, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,

    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    pub fn new(projection_raw: &'a PR) -> Self {
        Self {
            pd: PhantomData,
            projection_raw,
            stream: Box::new(STREAM::default()), // stub value
        }
    }
}

impl<'a, PR, STREAM, T> ResampleTrait for ResampleNone<'a, PR, STREAM, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
}

impl<'a, PR, STREAM, T> StreamIn for ResampleNone<'a, PR, STREAM, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SInput = STREAM;

    #[inline]
    fn stream_in(&mut self, stream: STREAM) {
        self.stream = Box::new(stream);
    }
}

impl<'a, PR, STREAM, T> Stream for ResampleNone<'a, PR, STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SC = Coordinate<T>;
    // #[inline]
    // fn get_dst(
    //     &self,
    // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
    // {
    //     self.stream.get_dst()
    // }

    #[inline]
    fn sphere(&mut self) {
        self.stream.sphere()
    }

    #[inline]
    fn line_start(&mut self) {
        self.stream.line_start()
    }

    #[inline]
    fn line_end(&mut self) {
        self.stream.line_end()
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.stream.polygon_start()
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.stream.polygon_end()
    }

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let t = &self.projection_raw.transform(&p);
        self.stream.point(&t, m);
    }
}
