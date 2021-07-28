use std::fmt::Display;
// use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use crate::clip::clip_sink_enum::ClipSinkEnum;
// use crate::projection::ProjectionRawTrait;
// use super::ResampleTrait;
use crate::stream::stream_in_trait::StreamCombo;
use crate::stream::stream_in_trait::StreamIn;
use crate::stream::Stream;
use crate::Transform;

// #[derive(Debug)]
pub struct ResampleNone<STREAM, T, TRANSFORMER>
where
    TRANSFORMER: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    STREAM: Stream<SC = Coordinate<T>>,
{
    // pd: PhantomData<&'a u8>,
    projection_raw: TRANSFORMER,
    /// Box to prevent infinite recusion.
    // pub stream: Box<ClipSinkEnum<'a, PR, T>>,
    pub stream: Box<STREAM>,
}

impl<STREAM, T, TRANSFORMER> ResampleNone<STREAM, T, TRANSFORMER>
where
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    TRANSFORMER: Transform<C = Coordinate<T>>,
{
    pub fn new(projection_raw: TRANSFORMER) -> ResampleNone<STREAM, T, TRANSFORMER> {
        Self {
            // pd: PhantomData,
            projection_raw,
            stream: Box::new(STREAM::default()), // stub value
        }
    }
}

// impl<PR, STREAM, T> ResampleTrait for ResampleNone<PR, STREAM, T>
// where
//     PR: Transform<C = Coordinate<T>>,
//     // Rc<PR>: Transform<C = Coordinate<T>>,
//     STREAM: Stream<SC = Coordinate<T>> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
// }

impl<'a, STREAM, T, TRANSFORMER> StreamCombo for ResampleNone<STREAM, T, TRANSFORMER>
where
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    TRANSFORMER: Transform<C = Coordinate<T>>,
{
}

impl<STREAM, T, TRANSFORMER> StreamIn for ResampleNone<STREAM, T, TRANSFORMER>
where
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    TRANSFORMER: Transform<C = Coordinate<T>>,
{
    type SInput = STREAM;

    #[inline]
    fn stream_in(&mut self, stream: STREAM) {
        self.stream = Box::new(stream);
    }
}

impl<STREAM, T, TRANSFORMER> Stream for ResampleNone<STREAM, T, TRANSFORMER>
where
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    TRANSFORMER: Transform<C = Coordinate<T>>,
{
    type SC = Coordinate<T>;

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
