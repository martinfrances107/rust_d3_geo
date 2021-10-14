use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use approx::AbsDiffEq;
use derivative::*;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use super::buffer::Buffer;
use super::compare_intersection::gen_compare_intersection;
use super::line_elem::LineElem;
use super::rejoin::rejoin;
use super::InterpolateFn;
use super::Line;
use super::PointVisible;

use crate::path::Result;
use crate::path::ResultEnum;
use crate::polygon_contains::polygon_contains;
use crate::projection::stream_node::StreamNode;
use crate::projection::stream_node_factory::StreamNodeFactory;
use crate::projection::NodeFactory;
use crate::stream::Stream;

use super::CleanState;

#[derive(Clone, Debug)]
enum PointFn {
    Default,
    Line,
    Ring,
}

#[derive(Clone, Debug)]
enum LineStartFn {
    Line,
    Ring,
}

#[derive(Clone, Debug)]
enum LineEndFn {
    Line,
    Ring,
}

/// State for a clipping node.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Clip<L, PV, SINK, T>
where
    L: Line,
    PV: PointVisible<T = T>,
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    line_node: StreamNode<L, SINK, T>,
    #[derivative(Debug = "ignore")]
    interpolate_fn: InterpolateFn<SINK, T>,

    /// A pipeline source node.
    pub ring_buffer: Rc<RefCell<Buffer<T>>>,
    pv: PV,
    start: LineElem<T>,
    polygon_started: bool,
    polygon: Vec<Vec<LineElem<T>>>,
    ring: Vec<LineElem<T>>,
    ring_sink_node: StreamNode<L, Buffer<T>, T>,
    segments: VecDeque<Vec<Vec<LineElem<T>>>>,
    point_fn: PointFn,
    line_start_fn: LineStartFn,
    line_end_fn: LineEndFn,
}

impl<L, PV, SINK, T> Clip<L, PV, SINK, T>
where
    L: Line,
    PV: PointVisible<T = T>,
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    pub(super) fn new(
        pv: PV,
        line_raw: L,
        interpolate_fn: InterpolateFn<SINK, T>,
        ring_buffer: Rc<RefCell<Buffer<T>>>,
        ring_sink_node: StreamNode<L, Buffer<T>, T>,
        sink: Rc<RefCell<SINK>>,
        start: LineElem<T>,
    ) -> Clip<L, PV, SINK, T> {
        let line_sink_factory: StreamNodeFactory<L, SINK, T> = StreamNodeFactory::new(line_raw);
        Clip {
            pv,
            line_node: line_sink_factory.generate(sink),
            interpolate_fn,
            start,

            polygon_started: false,
            polygon: Vec::new(),
            ring: Vec::new(),
            ring_sink_node,
            ring_buffer,
            segments: VecDeque::new(),

            // Cannot use 'point_fn' what is the default value?
            point_fn: PointFn::Default,
            line_start_fn: LineStartFn::Line,
            line_end_fn: LineEndFn::Line,
        }
    }
}

impl<L, PV, SINK, T> StreamNode<Clip<L, PV, SINK, T>, SINK, T>
where
    L: Line,
    PV: PointVisible<T = T>,
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    #[inline]
    pub(super) fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        if self.raw.pv.point_visible(p, None) {
            self.sink.borrow_mut().point(p, m);
        }
    }

    #[inline]
    fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.raw.line_node.sink.borrow_mut().point(p, m);
    }

    #[inline]
    fn line_start_default(&mut self) {
        self.raw.point_fn = PointFn::Line;
        self.raw.line_node.sink.borrow_mut().line_start();
    }

    #[inline]
    fn line_end_default(&mut self) {
        self.raw.point_fn = PointFn::Default;
        self.raw.line_node.sink.borrow_mut().line_end();
    }

    #[inline]
    fn point_ring(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.raw.ring.push(LineElem { p: *p, m });
        self.raw.ring_sink_node.sink.borrow_mut().point(p, m);
    }

    #[inline]
    fn ring_start(&mut self) {
        self.raw.ring_sink_node.sink.borrow_mut().line_start();
        self.raw.ring.clear();
    }

    fn ring_end(&mut self) {
        let le = self.raw.ring[0];
        // javascript version drops m here.
        self.point_ring(&le.p, None);
        self.raw.ring_sink_node.sink.borrow_mut().line_end();

        let clean = self.raw.ring_sink_node.raw.clean();

        let mut ring_segments = match self.raw.ring_sink_node.sink.borrow_mut().result() {
            Some(ResultEnum::BufferOutput(result)) => {
                // Can I find a way of doing this with the expense of dynamic conversion.
                result
            }
            Some(_) => {
                panic!("None buffer ");
            }
            None => panic!("was expecting something."),
        };

        let n = ring_segments.len();
        let m;

        self.raw.ring.pop();
        self.raw.polygon.push(self.raw.ring.clone());
        // in this javascript version this value is set to NULL
        // is my assumption that this is valid true?
        // self.ring = None;
        self.raw.ring.clear();

        if n == 0 {
            return;
        }

        // No intersections.
        match clean {
            CleanState::NoIntersections => {
                let segment = ring_segments
                    .pop_front()
                    .expect("We have previously checked that the .len() is >0 ( n ) ");
                m = segment.len() - 1;
                if m > 0 {
                    if !self.raw.polygon_started {
                        self.sink.borrow_mut().polygon_start();
                        self.raw.polygon_started = true;
                    }
                    self.sink.borrow_mut().line_start();
                    for s in segment.iter().take(m) {
                        let point = s.p;
                        self.sink.borrow_mut().point(&point, None);
                    }
                    self.sink.borrow_mut().line_end();
                }
                return;
            }
            CleanState::IntersectionsRejoin => {
                // Rejoin connected segments.
                // TODO reuse ringBuffer.rejoin()?
                if n > 1 {
                    let pb = [
                        ring_segments.pop_back().unwrap(),
                        ring_segments.pop_front().unwrap(),
                    ]
                    .concat();
                    ring_segments.push_back(pb);
                }
            }
            CleanState::IntersectionsOrEmpty => {
                // No-op
            }
            CleanState::Undefined => {
                panic!("must be defined by now.")
            }
        }

        let filtered: Vec<Vec<LineElem<T>>> = ring_segments
            .into_iter()
            .filter(|segment| segment.len() > 1)
            .collect();
        self.raw.segments.push_back(filtered);
    }
}

impl<L, PV, SINK, T> Stream for StreamNode<Clip<L, PV, SINK, T>, SINK, T>
where
    L: Line,
    PV: PointVisible<T = T>,
    SINK: Stream<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self.raw.point_fn {
            PointFn::Default => self.point_default(p, m),
            PointFn::Line => self.point_line(p, m),
            PointFn::Ring => self.point_ring(p, m),
        }
    }

    #[inline]
    fn line_start(&mut self) {
        match self.raw.line_start_fn {
            LineStartFn::Ring => self.ring_start(),
            LineStartFn::Line => self.line_start_default(),
        }
    }

    #[inline]
    fn line_end(&mut self) {
        match self.raw.line_end_fn {
            LineEndFn::Ring => self.ring_end(),
            LineEndFn::Line => self.line_end_default(),
        }
    }

    fn polygon_start(&mut self) {
        self.raw.point_fn = PointFn::Ring;
        self.raw.line_start_fn = LineStartFn::Ring;
        self.raw.line_end_fn = LineEndFn::Ring;
        self.raw.segments = VecDeque::new();
        self.raw.polygon = Vec::new();
    }

    fn polygon_end(&mut self) {
        self.raw.point_fn = PointFn::Default;
        self.raw.line_start_fn = LineStartFn::Line;
        self.raw.line_end_fn = LineEndFn::Line;

        let segments_inner: Vec<Vec<LineElem<T>>> =
            self.raw.segments.clone().into_iter().flatten().collect();
        let start_inside = polygon_contains(&self.raw.polygon, &self.raw.start);

        if !segments_inner.is_empty() {
            if !self.raw.polygon_started {
                self.raw.polygon_started = true;
            }

            rejoin(
                &segments_inner,
                gen_compare_intersection(),
                start_inside,
                self.raw.interpolate_fn.clone(),
                self.sink.clone(),
            );
        } else if start_inside {
            if !self.raw.polygon_started {
                self.raw.polygon_started = true;
            }
            self.sink.borrow_mut().line_start();
            (self.raw.interpolate_fn)(None, None, T::one(), self.sink.clone());
            self.sink.borrow_mut().line_end();
        };
        if self.raw.polygon_started {
            self.sink.borrow_mut().polygon_end();
            self.raw.polygon_started = false;
        }
        self.raw.segments.clear();
        self.raw.polygon.clear();
    }

    fn sphere(&mut self) {
        self.sink.borrow_mut().polygon_start();
        self.raw.line_node.sink.borrow_mut().line_start();
        (self.raw.interpolate_fn)(None, None, T::one(), self.sink.clone());
        self.sink.borrow_mut().line_end();
        self.sink.borrow_mut().polygon_end();
    }
}
