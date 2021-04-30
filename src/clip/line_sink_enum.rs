use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;

use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::buffer::ClipBuffer;
use super::clip_sink_enum::ClipSinkEnum;
use crate::path::PathResult;
use crate::path::PathResultEnum;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;

#[derive(Clone, Debug)]
pub enum LineSinkEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    CSE(ClipSinkEnum<T>),
    CB(ClipBuffer<T>),
}

impl<T> LineSinkEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    pub fn result(&mut self) -> Option<PathResultEnum<T>> {
        match self {
            LineSinkEnum::CB(l) => l.result(),
            LineSinkEnum::CSE(_) => {
                panic!("Calling result on a none buffer");
            }
        }
    }
}

impl<T> Stream<T> for LineSinkEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C = Coordinate<T>;

    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        match self {
            LineSinkEnum::CSE(cse) => cse.point(p, m),
            LineSinkEnum::CB(cb) => cb.point(p, m),
        }
    }
    fn sphere(&mut self) {
        match self {
            LineSinkEnum::CSE(cse) => cse.sphere(),
            LineSinkEnum::CB(cb) => cb.sphere(),
        }
    }
    fn line_start(&mut self) {
        match self {
            LineSinkEnum::CSE(cse) => cse.line_start(),
            LineSinkEnum::CB(cb) => cb.line_start(),
        }
    }
    fn line_end(&mut self) {
        match self {
            LineSinkEnum::CSE(cse) => cse.line_end(),
            LineSinkEnum::CB(cb) => cb.line_end(),
        }
    }
    fn polygon_start(&mut self) {
        match self {
            LineSinkEnum::CSE(cse) => cse.sphere(),
            LineSinkEnum::CB(cb) => cb.sphere(),
        }
    }
    fn polygon_end(&mut self) {
        match self {
            LineSinkEnum::CSE(cse) => cse.sphere(),
            LineSinkEnum::CB(cb) => cb.sphere(),
        }
    }
    fn get_dst(&self) -> StreamDst<T> {
        match self {
            LineSinkEnum::CSE(cse) => cse.get_dst(),
            LineSinkEnum::CB(cb) => cb.get_dst(),
        }
    }
}
