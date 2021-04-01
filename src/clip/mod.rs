pub mod antimeridian;
pub mod buffer;
pub mod circle;
pub mod clip;
pub mod clip_base;

mod rejoin;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use std::ops::AddAssign;

use crate::path::PathResult;
use crate::path::PathResultEnum;
use crate::projection::resample::ResampleEnum;
use crate::stream::Stream;
use crate::stream::StreamDst;
use crate::stream::{Clean, CleanEnum};

use antimeridian::line::Line as AntimeridianLine;
use antimeridian::ClipAntimeridian;
use buffer::ClipBuffer;
use circle::line::Line as CircleLine;
use circle::ClipCircle;
// use crate::clip::clip::ClipSinkEnum;

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

// impl<T> ClipSinkEnum<T>
// where
//     T: AddAssign + CoordFloat + Default + FloatConst,
// {
//     fn result(&mut self) -> Option<PathResultEnum<T>> {
//         todo!("what todo here?");
//         //    Some(PathResultEnum::Area(T::from(12345f64).unwrap()))
//     }
// }

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

#[derive(Clone, Debug)]
pub enum LineEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    Antimeridian(AntimeridianLine<T>),
    Circle(CircleLine<T>),
}

impl<T> LineEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    fn stream_in(&mut self, stream: LineSinkEnum<T>) {
        match self {
            LineEnum::Antimeridian(line) => line.stream_in(stream),
            LineEnum::Circle(line) => line.stream_in(stream),
        }
    }
    // deviation from javascript access to ring_buffer is through
    // ring_sink!
    #[inline]
    fn get_stream(&self) -> LineSinkEnum<T> {
        match self {
            LineEnum::Antimeridian(line) => line.get_stream(),
            LineEnum::Circle(line) => line.get_stream(),
        }
    }
}

impl<T> Clean for LineEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    fn clean(&self) -> CleanEnum {
        match self {
            LineEnum::Antimeridian(l) => l.clean(),
            LineEnum::Circle(l) => l.clean(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum LineSinkEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    CSE(ClipSinkEnum<T>),
    CB(ClipBuffer<T>),
}

impl<T> LineSinkEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    fn result(&mut self) -> Option<PathResultEnum<T>> {
        match self {
            LineSinkEnum::CB(l) => l.result(),
            LineSinkEnum::CSE(_) => {
                panic!("Calling result on a none buffer");
            }
        }
    }
}

impl<T> Stream<T> for LineEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    type C = Coordinate<T>;

    #[inline]
    fn get_dst(&self) -> StreamDst<T> {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.get_dst(),
            LineEnum::Circle(circle) => circle.get_dst(),
        }
    }

    #[inline]
    fn sphere(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.sphere(),
            LineEnum::Circle(circle) => circle.sphere(),
        }
    }

    #[inline]
    fn polygon_start(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.polygon_start(),
            LineEnum::Circle(circle) => circle.polygon_start(),
        }
    }

    #[inline]
    fn polygon_end(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.polygon_end(),
            LineEnum::Circle(circle) => circle.polygon_end(),
        }
    }

    #[inline]
    fn line_start(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.line_start(),
            LineEnum::Circle(circle) => circle.line_start(),
        }
    }

    #[inline]
    fn line_end(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.line_end(),
            LineEnum::Circle(circle) => circle.line_end(),
        }
    }

    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.point(p, m),
            LineEnum::Circle(circle) => circle.point(p, m),
            // LineEnum::Stub => panic!("calling point on a stub!"),
        }
    }
    //// Todo must connect up other stream functions. or find a better way.
}

#[derive(Clone, Debug)]
pub enum ClipRaw<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    Antimeridian(ClipAntimeridian<T>),
    Circle(ClipCircle<T>),
}

impl<T> Default for ClipRaw<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    fn default() -> Self {
        ClipRaw::Antimeridian(ClipAntimeridian::default())
    }
}

pub trait ClipTraitRaw<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type SctC;
    type SctOC;
    type SctT: CoordFloat + FloatConst;
    // type SctStream;
    type SctCi;

    fn point_visible(&self, _p: &Self::SctC, _z: Option<u8>) -> bool;

    // Intersections are sorted along the clip edge. For both antimeridian cutting
    // and circle clipPIng, the same comparison is used.
    fn compare_intersection(&self, _a: Self::SctCi, _b: Self::SctCi) -> Self::SctT {
        // let a_dashed = a.x;
        // let part1 = match a_dashed.x < Self::SctT::zero() {
        //     true => a_dashed.y - Self::SctT::FRAC_PI_2() - Self::SctT::epsilon(),
        //     false => Self::SctT::FRAC_PI_2() - a_dashed.y,
        // };
        // let b_dashed = b.x;
        // let part2 = match b_dashed.x < Self::SctT::zero() {
        //     true => b_dashed.y - Self::SctT::FRAC_PI_2() - Self::SctT::epsilon(),
        //     false => Self::SctT::FRAC_PI_2() - b_dashed.y,
        // };

        // return part1 - part2;
        panic!("why is this called.");
    }

    fn interpolate(
        &self,
        _from: Self::SctOC,
        _to: Self::SctOC,
        _direction: Self::SctT,
        _stream: impl Stream<T, C = Coordinate<T>>,
    );
}
