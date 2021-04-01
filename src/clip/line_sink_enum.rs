use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use std::ops::AddAssign;

use super::buffer::ClipBuffer;
use super::clip_sink_enum::ClipSinkEnum;
use crate::path::PathResult;
use crate::path::PathResultEnum;

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
    pub fn result(&mut self) -> Option<PathResultEnum<T>> {
        match self {
            LineSinkEnum::CB(l) => l.result(),
            LineSinkEnum::CSE(_) => {
                panic!("Calling result on a none buffer");
            }
        }
    }
}
