use std::collections::VecDeque;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::antimeridian::line::Line as AntimeridianLine;
use super::buffer::LineElem;
use super::clip_sink_enum::ClipSinkEnum;
use super::line_enum::LineEnum;

#[derive(Clone, Debug)]
pub struct ClipBase<T: AddAssign + CoordFloat + Default + FloatConst> {
    pub line: LineEnum<T>,
    pub polygon_started: bool,
    pub polygon: Vec<Vec<Coordinate<T>>>,
    pub ring: Vec<Coordinate<T>>,
    pub ring_sink: LineEnum<T>,
    pub segments: VecDeque<Vec<Vec<LineElem<T>>>>,
    pub start: Coordinate<T>,
    pub sink: ClipSinkEnum<T>,
}

impl<T> Default for ClipBase<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    fn default() -> Self {
        let mut segments = VecDeque::new();
        segments.push_front(vec![vec![]]);
        Self {
            line: LineEnum::Antimeridian(AntimeridianLine::default()),
            polygon_started: false,
            polygon: vec![vec![]],
            ring: vec![],
            ring_sink: LineEnum::Antimeridian(AntimeridianLine::default()),
            segments,
            sink: ClipSinkEnum::Blank,
            start: Coordinate {
                x: -T::PI(),
                y: -T::FRAC_PI_2(),
            },
        }
    }
}
