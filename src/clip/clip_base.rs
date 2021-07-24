use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use crate::projection::ProjectionRawTrait;
// use crate::stream::stream_dst::StreamDst;
// use crate::stream::stream_in_trait::StreamIn;
use crate::stream::Stream;
// use crate::Transform;

use super::clip_buffer::ClipBuffer;
// use super::line_trait::LineTrait;

// use super::clip_sink_enum::ClipSinkEnum;
use super::line_elem::LineElem;
use super::LCB;

// #[derive(Debug)]
pub struct ClipBase<L, SINK, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    // SINK: Stream<SC = Coordinate<T>>,
    // LB: LineTrait + StreamIn<SInput=ClipBuffer<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub line: L,
    pub polygon_started: bool,
    pub polygon: Vec<Vec<LineElem<T>>>,
    pub ring: Vec<LineElem<T>>,
    pub ring_sink: Box<dyn LCB<STREAM = ClipBuffer<T>, SC = Coordinate<T>>>,
    pub ring_buffer: Rc<RefCell<ClipBuffer<T>>>,
    pub segments: VecDeque<Vec<Vec<LineElem<T>>>>,
    pub start: LineElem<T>,

    // // #[derivative(Debug = "ignore")]
    // pub point_fn: fn(&mut Self, p: &Coordinate<T>, m: Option<u8>),
    // // #[derivative(Debug = "ignore")]
    // pub line_start_fn: fn(&mut Self),
    // // #[derivative(Debug = "ignore")]
    // pub line_end_fn: fn(&mut Self),
    pub use_point_line: bool,
    pub use_ring_start: bool,
    pub use_ring_end: bool,

    // Why Box?
    // sink will be passed into interpolate in a closure
    // and the closure signature does not supprt impl!!!
    // pub sink: Box<ClipSinkEnum<'a, PR, T>>,
    pub sink: Rc<RefCell<SINK>>,
}

impl<L, SINK, T> ClipBase<L, SINK, T>
where
    SINK: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn new(
        // projection_raw: PR,
        line: L,
        ring_buffer: Rc<RefCell<ClipBuffer<T>>>,
        ring_sink: Box<dyn LCB<SC = Coordinate<T>, STREAM = ClipBuffer<T>>>,
        start: LineElem<T>,
    ) -> Self
where
        // Rc<PR>: Transform<C = Coordinate<T>>,
        // PR: Transform<C = Coordinate<T>>,
    {
        let mut segments = VecDeque::new();
        segments.push_front(vec![vec![]]);
        Self {
            // pd: PhantomData,
            polygon_started: false,
            polygon: vec![vec![]],
            line,
            ring: vec![],
            ring_sink,
            ring_buffer,
            use_point_line: false,
            use_ring_start: false,
            use_ring_end: false,
            segments,
            // sink: Box::new(ClipSinkEnum::new(projection_raw)),
            sink: Rc::new(RefCell::new(SINK::default())),
            start,
        }
    }
}
