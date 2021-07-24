// use geo::CoordFloat;
// use std::fmt::Display;
// use std::ops::AddAssign;
// use std::rc::Rc;

// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::clip::clip_sink_enum::ClipSinkEnum;
// use crate::stream::Stream;
// use crate::Transform;

// Methods need by rejoin to function.
// 'sink' is the stream on which the clip types operates on
// when the buffering is complete. ( See ClipX::polygon_end())
pub trait Interpolate // where
//     <Self as Interpolate>::IStream: Stream<SC = Self::IC>,
{
    type IT;
    type IC;
    // type IStream;

    // fn get_sink(&mut self) -> &mut Self::IStream;
    fn interpolate(
        &mut self,
        to: Option<Self::IC>,
        from: Option<Self::IC>,
        dir: Self::IT,
        // stream: &mut Self::IStream,
    );
}
