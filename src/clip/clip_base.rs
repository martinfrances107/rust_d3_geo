use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::line_elem::LineElem;
use super::clip_sink_enum::ClipSinkEnum;
use super::line_enum::LineEnum;

#[derive(Clone, Debug)]
pub struct ClipBase<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> {
    pub line: LineEnum<T>,
    pub polygon_started: bool,
    pub polygon: Vec<Vec<LineElem<T>>>,
    pub ring: Vec<LineElem<T>>,
    pub ring_sink: LineEnum<T>,
    pub segments: VecDeque<Vec<Vec<LineElem<T>>>>,
    pub start: LineElem<T>,
    pub sink: ClipSinkEnum<T>,
}

impl<T> Default for ClipBase<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    fn default() -> Self {
        let mut segments = VecDeque::new();
        segments.push_front(vec![vec![]]);
        Self {
            line: LineEnum::Blank,
            polygon_started: false,
            polygon: vec![vec![]],
            ring: vec![],
            ring_sink: LineEnum::Blank,
            segments,
            sink: ClipSinkEnum::Blank,
            start: LineElem {
                p: Coordinate {
                    x: -T::PI(),
                    y: -T::FRAC_PI_2(),
                },
                m: None,
            },
        }
    }
}
