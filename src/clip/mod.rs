pub mod antimeridian;
pub mod buffer;
pub mod circle;
pub mod clip;
pub mod clip_base;

mod rejoin;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::resample::ResampleEnum;
use crate::stream::Stream;

use crate::stream::StreamSrc;

use antimeridian::line::Line as AntimeridianLine;
use antimeridian::ClipAntimeridian;
use buffer::ClipBuffer;
use circle::line::Line as CircleLine;
use circle::ClipCircle;
// use crate::clip::clip::ClipSinkEnum;

/// Wrapper for stream inputs to Clip.
#[derive(Clone)]
pub enum ClipSinkEnum<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    Resample(ResampleEnum<T>),
    Src(StreamSrc<T>),
}

#[derive(Clone)]
pub enum LineEnum<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    Antimeridian(AntimeridianLine<T>),
    Circle(CircleLine<T>),
    // Stub, // initial, default undefined state
}

#[derive(Clone)]
pub enum LineSinkEnum<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    CSE(ClipSinkEnum<T>),
    CB(ClipBuffer<T>),
}

impl<T> Stream for LineEnum<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
    fn point(&mut self, p: Self::C, m: Option<u8>) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.point(p, m),
            LineEnum::Circle(circle) => circle.point(p, m),
            // LineEnum::Stub => panic!("calling point on a stub!"),
        }
    }
    //// Todo must connect up other stream functions. or find a better way.
}

#[derive(Clone)]
pub enum ClipRaw<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    Antimeridian(ClipAntimeridian<T>),
    Circle(ClipCircle<T>),
}

// pub trait BufferInTrait {
//     type BitCB;
//     fn buffer_in(&mut self, &buffer: Self::BitCB);
// }

pub trait ClipTraitRaw {
    type SctC;
    type SctOC;
    type SctT: CoordFloat + FloatConst;
    type SctStream;
    type SctCi;

    fn point_visible(&self, _p: Self::SctC, _z: Option<u8>) -> bool;

    // fn clip_line(&self, stream: StreamPathResultNode<T>) -> StreamCleanNode<T>;
    // Intersections are sorted along the clip edge. For both antimeridian cutting
    // and circle clipPIng, the same comparison is used.
    fn compare_intersection(&self, a: Self::SctCi, b: Self::SctCi) -> Self::SctT {
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
        _stream: Self::SctStream,
    );
}
