use geo::CoordFloat;
use std::fmt::Display;
use std::ops::AddAssign;
// use std::rc::Rc;

// use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use crate::clip::clip_sink_enum::ClipSinkEnum;
use crate::stream::Stream;
// use crate::Transform;
pub trait Interpolate // Rc<<Self as Interpolate>::IPR>: Transform,
{
    type IT;
    type IC;
    type IStream;

    fn interpolate(
        &self,
    ) -> Box<dyn Fn(Option<Self::IC>, Option<Self::IC>, Self::IT, &mut Self::IStream) + '_>
    where
        <Self as Interpolate>::IStream: Stream<SC = Self::IC>,
        <Self as Interpolate>::IT: AddAssign
            + AsPrimitive<<Self as Interpolate>::IT>
            + CoordFloat
            + Default
            + Display
            + FloatConst;
}
