use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::stream_dst::StreamDst;
use crate::stream::Clean;
use crate::stream::CleanEnum;
use crate::stream::Stream;

use super::antimeridian::line::Line as AntimeridianLine;
use super::circle::line::Line as CircleLine;
use super::line_sink_enum::LineSinkEnum;

#[derive(Clone, Debug)]
pub enum LineEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    Antimeridian(AntimeridianLine<T>),
    Circle(CircleLine<T>),
    Blank,
}

impl<T> LineEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    pub fn stream_in(&mut self, stream: LineSinkEnum<T>) {
        match self {
            LineEnum::Antimeridian(line) => line.stream_in(stream),
            LineEnum::Circle(line) => line.stream_in(stream),
            LineEnum::Blank => panic!("LineEnum stream_in Shoud not be injecting a blank"),
        }
    }
    // deviation from javascript access to ring_buffer is through
    // ring_sink!
    #[inline]
    pub fn get_stream(&mut self) -> &mut LineSinkEnum<T> {
        match self {
            LineEnum::Antimeridian(line) => line.get_stream(),
            LineEnum::Circle(line) => line.get_stream(),
            LineEnum::Blank => panic!("LineEnum get_stream Should not be returning from  a blank."),
        }
    }
}

impl<T> Clean for LineEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    fn clean(&self) -> CleanEnum {
        match self {
            LineEnum::Antimeridian(l) => l.clean(),
            LineEnum::Circle(l) => l.clean(),
            LineEnum::Blank => panic!("should not be cleaning a blank."),
        }
    }
}

impl<T> Stream<T> for LineEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C = Coordinate<T>;

    #[inline]
    fn get_dst(&self) -> StreamDst<T> {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.get_dst(),
            LineEnum::Circle(circle) => circle.get_dst(),
            LineEnum::Blank => panic!("blank has no destinaation"),
        }
    }

    #[inline]
    fn sphere(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.sphere(),
            LineEnum::Circle(circle) => circle.sphere(),
            LineEnum::Blank => panic!("blank -- sphere!"),
        }
    }

    #[inline]
    fn polygon_start(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.polygon_start(),
            LineEnum::Circle(circle) => circle.polygon_start(),
            LineEnum::Blank => panic!("blank -- polygon start!"),
        }
    }

    #[inline]
    fn polygon_end(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.polygon_end(),
            LineEnum::Circle(circle) => circle.polygon_end(),
            LineEnum::Blank => panic!("blank -- polygon end!"),
        }
    }

    #[inline]
    fn line_start(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.line_start(),
            LineEnum::Circle(circle) => circle.line_start(),
            LineEnum::Blank => panic!("blank -- line start!"),
        }
    }

    #[inline]
    fn line_end(&mut self) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.line_end(),
            LineEnum::Circle(circle) => circle.line_end(),
            LineEnum::Blank => panic!("blank -- line end!"),
        }
    }

    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        match self {
            LineEnum::Antimeridian(antimeridian) => antimeridian.point(p, m),
            LineEnum::Circle(circle) => circle.point(p, m),
            LineEnum::Blank => panic!("blank -- point!"),
        }
    }
}
