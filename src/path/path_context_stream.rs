use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::path_context::PathContext;
use super::path_string::PathString;
use super::PathResult;
use super::PathResultEnum;
use super::PointRadiusTrait;
// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;

#[derive(Clone, Debug)]
pub enum PathContextStream<T>
where
    T: AsPrimitive<T> + Display,
{
    PC(PathContext<T>),
    PS(PathString<T>),
}

impl<T> PathResult for PathContextStream<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Self::Out {
        match self {
            PathContextStream::PC(pc) => pc.result(),
            PathContextStream::PS(ps) => ps.result(),
        }
    }
}

impl<T> PointRadiusTrait for PathContextStream<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type PrtT = Option<T>;
    fn point_radius(&mut self, val: Self::PrtT) {
        match self {
            PathContextStream::PC(pc) => pc.point_radius(val),
            PathContextStream::PS(ps) => ps.point_radius(val),
        }
    }
}

impl<T> Stream for PathContextStream<T>
where
    T: CoordFloat + FloatConst + AddAssign + AsPrimitive<T> + Default + Display,
{
    type SC = Coordinate<T>;

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self {
            PathContextStream::PC(pc) => pc.point(p, m),
            PathContextStream::PS(ps) => ps.point(p, m),
        }
    }
    fn sphere(&mut self) {
        match self {
            PathContextStream::PC(pc) => pc.sphere(),
            PathContextStream::PS(ps) => ps.sphere(),
        }
    }
    fn line_start(&mut self) {
        match self {
            PathContextStream::PC(pc) => pc.line_start(),
            PathContextStream::PS(ps) => ps.line_start(),
        }
    }
    fn line_end(&mut self) {
        match self {
            PathContextStream::PC(pc) => pc.line_end(),
            PathContextStream::PS(ps) => ps.line_end(),
        }
    }
    fn polygon_start(&mut self) {
        match self {
            PathContextStream::PC(pc) => pc.polygon_start(),
            PathContextStream::PS(ps) => ps.polygon_start(),
        }
    }
    fn polygon_end(&mut self) {
        match self {
            PathContextStream::PC(pc) => pc.polygon_end(),
            PathContextStream::PS(ps) => ps.polygon_end(),
        }
    }
    // fn get_dst(&self) -> Self {
    //     match self {
    //         PathContextStream::PC(pc) => pc.get_dst(),
    //         PathContextStream::PS(ps) => ps.get_dst(),
    //     }
    // }
}
