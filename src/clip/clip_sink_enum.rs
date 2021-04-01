use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use std::ops::AddAssign;

use crate::projection::resample::ResampleEnum;
use crate::stream::Stream;
use crate::stream::StreamDst;

/// Wrapper for stream inputs to Clip.
#[derive(Clone, Debug)]
pub enum ClipSinkEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    Resample(ResampleEnum<T>),
    Src(StreamDst<T>),
    Blank,
}

impl<T> Stream<T> for ClipSinkEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    type C = Coordinate<T>;

    #[inline]
    fn get_dst(&self) -> StreamDst<T> {
        match self {
            ClipSinkEnum::Resample(r) => r.get_dst(),
            ClipSinkEnum::Src(s) => s.get_dst(),
            ClipSinkEnum::Blank => panic!("calling get_dst on a blank"),
        }
    }

    #[inline]
    fn sphere(&mut self) {
        match self {
            ClipSinkEnum::Resample(r) => r.sphere(),
            ClipSinkEnum::Src(s) => s.sphere(),
            ClipSinkEnum::Blank => panic!("calling sphere on a blank"),
        }
    }

    #[inline]
    fn polygon_start(&mut self) {
        match self {
            ClipSinkEnum::Resample(r) => r.polygon_start(),
            ClipSinkEnum::Src(s) => s.polygon_start(),
            ClipSinkEnum::Blank => panic!("calling polygon_start on a blank"),
        }
    }

    #[inline]
    fn polygon_end(&mut self) {
        match self {
            ClipSinkEnum::Resample(r) => r.polygon_end(),
            ClipSinkEnum::Src(s) => s.polygon_end(),
            ClipSinkEnum::Blank => panic!("calling polygon_end on a blank"),
        }
    }

    #[inline]
    fn line_start(&mut self) {
        match self {
            ClipSinkEnum::Resample(r) => r.line_start(),
            ClipSinkEnum::Src(s) => s.line_start(),
            ClipSinkEnum::Blank => panic!("calling line_start on a blank"),
        }
    }

    #[inline]
    fn line_end(&mut self) {
        match self {
            ClipSinkEnum::Resample(r) => r.line_end(),
            ClipSinkEnum::Src(s) => s.line_end(),
            ClipSinkEnum::Blank => panic!("calling line_end on a blank"),
        }
    }

    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        match self {
            ClipSinkEnum::Resample(r) => r.point(p, m),
            ClipSinkEnum::Src(s) => s.point(p, m), // LineEnum::Stub => panic!("calling point on a stub!"),
            ClipSinkEnum::Blank => panic!("calling point on a blank"),
        }
    }
    //// Todo must connect up other stream functions. or find a better way.
}
