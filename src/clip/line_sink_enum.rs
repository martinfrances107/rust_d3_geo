use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::path::PathResult;
use crate::path::PathResultEnum;

use super::buffer::ClipBuffer;
use super::clip_sink_enum::ClipSinkEnum;

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
