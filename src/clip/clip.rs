use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use derivative::Derivative;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use super::buffer::Buffer;
use super::compare_intersections::compare_intersections;
use super::line_elem::LineElem;
use super::rejoin::rejoin;
use super::InterpolateFn;
use super::Line;
use super::PointVisible;

use crate::path::Result;
use crate::path::ResultEnum;
use crate::projection::stream_node::StreamNode;
use crate::projection::stream_node_factory::StreamNodeFactory;
use crate::projection::NodeFactory;
use crate::stream::Stream;

use super::CleanEnum;

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

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Clip<L, PV, SINK, T>
where
    L: Line,
    PV: PointVisible<T = T>,
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    pub line_node: StreamNode<L, SINK, T>,
    #[derivative(Debug = "ignore")]
    pub interpolate_fn: InterpolateFn<SINK, T>,

    /// A pipeline source node.
    pub ring_buffer: Rc<RefCell<Buffer<T>>>,
    pub pv: PV,
    start: LineElem<T>,
    pub polygon_started: bool,
    pub polygon: Vec<Vec<LineElem<T>>>,
    pub ring: Vec<LineElem<T>>,
    pub ring_sink_node: StreamNode<L, Buffer<T>, T>,
    pub segments: VecDeque<Vec<Vec<LineElem<T>>>>,
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
    pub fn new(
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
            line_start_fn: LineStartFn::Ring,
            line_end_fn: LineEndFn::Ring,
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
        println!("clip point_default");
        if self.raw.pv.point_visible(p, None) {
            self.sink.borrow_mut().point(p, m);
        }
    }

    #[inline]
    fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("clip point_line");
        self.raw.line_node.sink.borrow_mut().point(p, m);
    }

    #[inline]
    fn line_start_default(&mut self) {
        println!("clip line_start_default");
        self.raw.point_fn = PointFn::Line;
        self.raw.line_node.sink.borrow_mut().line_start();
    }

    #[inline]
    fn line_end_default(&mut self) {
        println!("clip line_end_default");
        self.raw.point_fn = PointFn::Default;
        self.raw.line_node.sink.borrow_mut().line_end();
    }

    #[inline]
    fn point_ring(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("clip point_ring {:?} {:?}", p, m);
        // println!("about to ring/push - ring_sink ");
        // println!("self.base {:#?} ", self.base.ring_sink);
        self.raw.ring.push(LineElem { p: *p, m });
        self.raw.ring_sink_node.sink.borrow_mut().point(p, m);
        println!("clip point_ring -- end");
    }

    #[inline]
    fn ring_start(&mut self) {
        println!("clip ring_start");
        self.raw.ring_sink_node.sink.borrow_mut().line_start();
        self.raw.ring.clear();
        println!("end clip ring_start");
    }

    fn ring_end(&mut self) {
        println!("clip ring_end  entry {:#?}", self.raw.ring);
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

        println!("clip ring_end() - ring segments {:#?}", ring_segments);
        // panic!("ring_end buffer result");
        let n = ring_segments.len();
        let m;
        // let mut point: Coordinate<T>;

        self.raw.ring.pop();
        // self.base.polygon.push(self.base.ring.clone());
        self.raw.polygon.push(self.raw.ring.clone());
        // in this javascript version this value is set to NULL
        // is my assumption that this is valid true?
        // self.ring = None;
        self.raw.ring = Vec::new();
        // self.raw.ring_reset();

        if n == 0 {
            return;
        }
        println!("no intersections n, c {:?} {:?}", n, clean);
        // No intersections.
        match clean {
            CleanEnum::NoIntersections => {
                println!("about to clean good path");
                // panic!("on the good path");
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
                    for i in 0..m {
                        let point = segment[i].p;
                        self.sink.borrow_mut().point(&point, None);
                    }
                    self.sink.borrow_mut().line_end();
                }
                return;
            }
            CleanEnum::IntersectionsRejoin => {
                // Rejoin connected segments.
                // TODO reuse ringBuffer.rejoin()?
                if n > 1 {
                    println!("funny buisness");
                    println!("ring_segemtns before fb {:#?}", ring_segments);
                    let pb = [
                        ring_segments.pop_back().unwrap(),
                        ring_segments.pop_front().unwrap(),
                    ]
                    .concat();
                    ring_segments.push_back(pb);
                }
            }
            CleanEnum::IntersectionsOrEmpty => {
                // No-op
            }
            CleanEnum::Undefined => {
                panic!("must be defined by now.")
            }
        }
        println!("final segments before filter {:#?}", ring_segments);
        // panic!("final segments");
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
    T: CoordFloat + FloatConst,
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
        println!("clip  polygon start");
        self.raw.point_fn = PointFn::Ring;
        self.raw.line_start_fn = LineStartFn::Ring;
        self.raw.line_end_fn = LineEndFn::Ring;
        self.raw.segments.clear();
        self.raw.polygon.clear();
    }
    fn polygon_end(&mut self) {
        println!("clip polygon_end");
        self.raw.point_fn = PointFn::Default;
        self.raw.line_start_fn = LineStartFn::Line;
        self.raw.line_end_fn = LineEndFn::Line;
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

            rejoin(
                &segments_merged,
                compare_intersections,
                start_inside,
                self.raw.interpolate_fn.clone(),
                self.sink.clone(),
            );
        } else if start_inside {
            if !self.raw.polygon_started {
                // self.base.sink.polygon_start();
                self.raw.polygon_started = true;
            }
            self.sink.borrow_mut().line_start();
            (self.raw.interpolate_fn)(None, None, T::one(), self.sink.clone());
            self.sink.borrow_mut().line_end();
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
        self.sink.borrow_mut().polygon_start();
        self.raw.line_node.sink.borrow_mut().line_start();
        (self.raw.interpolate_fn)(None, None, T::one(), self.sink.clone());
        self.sink.borrow_mut().line_end();
        self.sink.borrow_mut().polygon_end();
    }
}
