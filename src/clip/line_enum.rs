use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use std::ops::AddAssign;

use super::antimeridian::line::Line as AntimeridianLine;
use super::circle::line::Line as CircleLine;
use super::line_sink_enum::LineSinkEnum;
use crate::stream::Stream;
use crate::stream::StreamDst;
use crate::stream::{Clean, CleanEnum};

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
    pub fn stream_in(&mut self, stream: LineSinkEnum<T>) {
        match self {
            LineEnum::Antimeridian(line) => line.stream_in(stream),
            LineEnum::Circle(line) => line.stream_in(stream),
        }
    }
    // deviation from javascript access to ring_buffer is through
    // ring_sink!
    #[inline]
    pub fn get_stream(&self) -> LineSinkEnum<T> {
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
