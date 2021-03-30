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

use crate::projection::resample::ResampleEnum;
use crate::stream::Stream;

use crate::stream::StreamDst;

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
    T: CoordFloat + Default + FloatConst,
{
    Resample(ResampleEnum<T>),
    Src(StreamDst<T>),
    Blank,
}

#[derive(Clone, Debug)]
pub enum LineEnum<T>
where
    T: CoordFloat + Default + FloatConst,
{
    Antimeridian(AntimeridianLine<T>),
    Circle(CircleLine<T>),
    // Stub, // initial, default undefined state
}

#[derive(Clone, Debug)]
pub enum LineSinkEnum<T>
where
    T: CoordFloat + Default + FloatConst,
{
    CSE(ClipSinkEnum<T>),
    CB(ClipBuffer<T>),
}

impl<T> Stream<T> for LineEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    type C = Coordinate<T>;

    fn get_dst(&self) -> StreamDst<T> {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.get_dst(),
            LineEnum::Circle(circle) => circle.get_dst(),
        }
    }
    fn sphere(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.sphere(),
            LineEnum::Circle(circle) => circle.sphere(),
        }
    }
    fn polygon_start(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.polygon_start(),
            LineEnum::Circle(circle) => circle.polygon_start(),
        }
    }
    fn polygon_end(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.polygon_end(),
            LineEnum::Circle(circle) => circle.polygon_end(),
        }
    }
    fn line_start(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.line_start(),
            LineEnum::Circle(circle) => circle.line_start(),
        }
    }
    fn line_end(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.line_end(),
            LineEnum::Circle(circle) => circle.line_end(),
        }
    }
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
    T: CoordFloat + Default + FloatConst,
{
    Antimeridian(ClipAntimeridian<T>),
    Circle(ClipCircle<T>),
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
