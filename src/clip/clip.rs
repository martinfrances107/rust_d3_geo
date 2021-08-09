use crate::clip::InterpolateRaw;
use crate::clip::InterpolateTrait;
use crate::clip::LineRaw;
use crate::clip::PointVisible;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::line_elem::LineElem;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

#[derive(Clone, Debug)]
pub struct Clip<I, L, PV, SINK, T>
where
    I: InterpolateRaw,
    L: LineRaw,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Nodes.
    pub line_node: Rc<RefCell<StreamNode<L, SINK, T>>>,
    pub interpolate_node: Rc<RefCell<StreamNode<I, SINK, T>>>,
    pub ring_buffer_node: Rc<RefCell<Buffer<T>>>,

    pub pv: PV,
    start: LineElem<T>,
    pub polygon_started: bool,
    pub polygon: Vec<Vec<LineElem<T>>>,
    pub ring: Vec<LineElem<T>>,
    // pub ring_sink: Rc<RefCell<dyn Stream<SC = Coordinate<T>>>>,
    pub segments: VecDeque<Vec<Vec<LineElem<T>>>>,

    // // #[derivative(Debug = "ignore")]
    // pub point_fn: fn(&mut Self, p: &Coordinate<T>, m: Option<u8>),
    // // #[derivative(Debug = "ignore")]
    // pub line_start_fn: fn(&mut Self),
    // // #[derivative(Debug = "ignore")]
    // pub line_end_fn: fn(&mut Self),
    pub use_point_line: bool,
    pub use_ring_start: bool,
    pub use_ring_end: bool,
}

impl<'a, I, L, PV, SINK, T> Stream for StreamNode<Clip<I, L, PV, SINK, T>, SINK, T>
where
    I: InterpolateRaw,
    L: LineRaw,
    PV: PointVisible,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;

    #[inline]
    fn point(&mut self, _p: &Coordinate<T>, _m: Option<u8>) {
        // (self.point_fn)(self, p, m);
    }

    #[inline]
    fn line_start(&mut self) {
        // (self.line_start_fn)(self);
    }

    #[inline]
    fn line_end(&mut self) {
        // (self.line_end_fn)(self);
    }

    fn polygon_start(&mut self) {
        println!("clip  polygon start");
        // self.point_fn = Self::point_ring;
        // self.line_start_fn = Self::ring_start;
        // self.line_end_fn = Self::ring_end;
        self.raw.segments.clear();
        self.raw.polygon.clear();
    }
    fn polygon_end(&mut self) {
        println!("clip polygon_end");
        // self.point_fn = Self::point_default;
        // self.line_start_fn = Self::line_start_default;
        // self.line_end_fn = Self::line_end_default;
        println!("about to merge {:#?}", self.raw.segments);
        let segments_merged: Vec<Vec<LineElem<T>>> =
            self.raw.segments.clone().into_iter().flatten().collect();
        // let start_inside = contains(&self.base.polygon, &self.base.start);
        let start_inside = true;

        if !segments_merged.is_empty() {
            println!("merged is not empty {:#?}", self.raw.segments);
            // panic!("pause here");
            if !self.raw.polygon_started {
                // self.base.sink.polygon_start();
                self.raw.polygon_started = true;
            }
            println!("into rejoin this path");

            // self.rejoin(
            //     &segments_merged,
            //     compare_intersections,
            //     start_inside,
            //     // self,
            //     // self.interpolate(),
            //     // &mut self.base.sink,
            // );
        } else if start_inside {
            if !self.raw.polygon_started {
                // self.base.sink.polygon_start();
                self.raw.polygon_started = true;
            }
            // self.base.sink.line_start();
            self.raw
                .interpolate_node
                .borrow_mut()
                .interpolate(None, None, T::one());
            // self.base.sink.line_end();
        };
        if self.raw.polygon_started {
            // self.base.sink.polygon_end();
            self.raw.polygon_started = false;
        }
        self.raw.segments.clear();
        self.raw.polygon.clear();
        println!("clip polygon_end -- exit");
    }

    fn sphere(&mut self) {
        // self.base.sink.polygon_start();
        // self.base.sink.line_start();
        // self.interpolate(None, None, T::one(), &mut self.base.sink);
        self.raw
            .interpolate_node
            .borrow_mut()
            .interpolate(None, None, T::one());
        // self.base.sink.line_end();
        // self.base.sink.polygon_end();
    }
}
