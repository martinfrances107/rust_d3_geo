use std::collections::VecDeque;
use std::fmt::Debug;

use approx::AbsDiffEq;
use derivative::*;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::Clean;
use crate::clip::Line;
use crate::path::Result;
use crate::path::ResultEnum;
use crate::polygon_contains::polygon_contains;
use crate::projection::stream_node::StreamNode;
use crate::projection::stream_node_factory::StreamNodeFactory;
use crate::projection::NodeFactory;
use crate::stream::Stream;

use super::buffer::Buffer;
use super::compare_intersection::gen_compare_intersection;
use super::line_elem::LineElem;
use super::rejoin::rejoin;
// use super::stream_node_line_factory::StreamNodeLineFactory;
use super::CleanState;
use super::InterpolateFn;
use super::PointVisible;

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

/// ClipNode is a special case of StreamNode
/// because of the way stream in held internally in
/// within a line node.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct ClipNode<EP, LINE, PV, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    LINE: Line,
    StreamNode<EP, LINE, SINK, T>: Stream<EP = EP, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PV: PointVisible<T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    line_node: StreamNode<EP, LINE, SINK, T>,
    #[derivative(Debug = "ignore")]
    interpolate_fn: InterpolateFn<SINK, T>,

    /// A pipeline source node.
    // pub ring_buffer: Buffer<T>,
    pv: PV,
    start: Coordinate<T>,
    polygon_started: bool,
    polygon: Vec<Vec<Coordinate<T>>>,
    ring: Vec<Coordinate<T>>,
    ring_sink_node: StreamNode<Buffer<T>, LINE, Buffer<T>, T>,
    segments: VecDeque<VecDeque<Vec<LineElem<T>>>>,
    point_fn: PointFn,
    line_start_fn: LineStartFn,
    line_end_fn: LineEndFn,
}

impl<EP, LINE, PV, SINK, T> ClipNode<EP, LINE, PV, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    LINE: Line,
    StreamNode<EP, LINE, SINK, T>: Stream<EP = EP, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PV: PointVisible<T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    /// Takes a line and cuts into visible segments. Return values used for polygon
    pub(super) fn new(
        pv: PV,
        stream_node_line_factory: StreamNodeFactory<EP, LINE, SINK, T>,
        interpolate_fn: InterpolateFn<SINK, T>,
        ring_sink_node: StreamNode<Buffer<T>, LINE, Buffer<T>, T>,
        sink: SINK,
        start: Coordinate<T>,
    ) -> ClipNode<EP, LINE, PV, SINK, T> {
        ClipNode {
            pv,
            line_node: stream_node_line_factory.generate(sink),
            interpolate_fn,
            start,

            polygon_started: false,
            polygon: Vec::new(),
            ring: Vec::new(),
            ring_sink_node,
            // ring_buffer,
            segments: VecDeque::new(),

            // Cannot use 'point_fn' what is the default value?
            point_fn: PointFn::Default,
            line_start_fn: LineStartFn::Line,
            line_end_fn: LineEndFn::Line,
        }
    }
}

impl<EP, LINE, PV, SINK, T> ClipNode<EP, LINE, PV, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    LINE: Line,
    StreamNode<EP, LINE, SINK, T>: Stream<EP = EP, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PV: PointVisible<T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    #[inline]
    pub(super) fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        if self.pv.point_visible(p) {
            self.line_node.sink.point(p, m);
        }
    }

    #[inline]
    fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.line_node.point(p, m);
    }

    #[inline]
    fn line_start_default(&mut self) {
        self.point_fn = PointFn::Line;
        self.line_node.line_start();
    }

    #[inline]
    fn line_end_default(&mut self) {
        self.point_fn = PointFn::Default;
        self.line_node.line_end();
    }

    #[inline]
    fn point_ring(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        self.ring.push(*p);
        self.ring_sink_node.point(p, _m);
    }

    #[inline]
    fn ring_start(&mut self) {
        self.ring_sink_node.line_start();
        self.ring.clear();
    }

    fn ring_end(&mut self) {
        let le = self.ring[0];
        // javascript version drops m here.
        self.point_ring(&le, None);
        self.ring_sink_node.line_end();

        // let clean = match &self.raw.ring_sink_node {
        //     LineNode::A(l) => l.raw.clean(),
        //     LineNode::C(l) => l.raw.clean(),
        // };
        let clean = self.ring_sink_node.raw.clean();

        // let ring_segments_result_o = match &mut self.raw.ring_sink_node {
        //     LineNode::A(l) => l.sink.result(),
        //     LineNode::C(l) => l.sink.result(),
        // };
        let ring_segments_result_o = self.ring_sink_node.sink.result();

        let mut ring_segments = match ring_segments_result_o {
            Some(ResultEnum::BufferOutput(result)) => result,
            Some(_) => {
                panic!("None buffer ");
            }
            None => panic!("was expecting something."),
        };

        let n = ring_segments.len();
        let m;

        self.ring.pop();
        self.polygon.push(self.ring.clone());
        // in this javascript version this value is set to NULL
        // is my assumption that this is valid true?
        // self.ring = None;
        self.ring.clear();

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
                    if !self.polygon_started {
                        self.line_node.sink.polygon_start();
                        self.polygon_started = true;
                    }
                    self.line_node.sink.line_start();
                    for s in segment.iter().take(m) {
                        let point = s.p;
                        self.line_node.sink.point(&point, None);
                    }
                    self.line_node.sink.line_end();
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
        }

        ring_segments.retain(|segment| segment.len() > 1);

        self.segments.push_back(ring_segments);
    }
}

impl<EP, LINE, PV, SINK, T> Stream for ClipNode<EP, LINE, PV, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    LINE: Line,
    StreamNode<EP, LINE, SINK, T>: Stream<EP = EP, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PV: PointVisible<T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type EP = EP;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        self.line_node.sink.get_endpoint()
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self.point_fn {
            PointFn::Default => self.point_default(p, m),
            PointFn::Line => self.point_line(p, m),
            PointFn::Ring => self.point_ring(p, m),
        }
    }

    #[inline]
    fn line_start(&mut self) {
        match self.line_start_fn {
            LineStartFn::Ring => self.ring_start(),
            LineStartFn::Line => self.line_start_default(),
        }
    }

    #[inline]
    fn line_end(&mut self) {
        match self.line_end_fn {
            LineEndFn::Ring => self.ring_end(),
            LineEndFn::Line => self.line_end_default(),
        }
    }

    fn polygon_start(&mut self) {
        self.point_fn = PointFn::Ring;
        self.line_start_fn = LineStartFn::Ring;
        self.line_end_fn = LineEndFn::Ring;
        self.segments = VecDeque::new();
        self.polygon = Vec::new();
    }

    fn polygon_end(&mut self) {
        self.point_fn = PointFn::Default;
        self.line_start_fn = LineStartFn::Line;
        self.line_end_fn = LineEndFn::Line;

        let segments_inner: Vec<Vec<LineElem<T>>> =
            self.segments.clone().into_iter().flatten().collect();

        let start_inside = polygon_contains(&self.polygon, &self.start);

        if !segments_inner.is_empty() {
            self.line_node.sink.polygon_start();
            if !self.polygon_started {
                self.polygon_started = true;
            }
            rejoin(
                &segments_inner,
                gen_compare_intersection(),
                start_inside,
                self.interpolate_fn.clone(),
                &mut self.line_node.sink,
            );
        } else if start_inside {
            if !self.polygon_started {
                self.line_node.sink.polygon_start();
                self.polygon_started = true;
            }
            self.line_node.sink.line_start();
            (self.interpolate_fn)(None, None, T::one(), &mut self.line_node.sink);
            self.line_node.sink.line_end();
        };
        if self.polygon_started {
            self.line_node.sink.polygon_end();
            self.polygon_started = false;
        }
        self.segments.clear();
        self.polygon.clear();
    }

    fn sphere(&mut self) {
        self.line_node.sink.polygon_start();
        self.line_node.sink.line_start();
        (self.interpolate_fn)(None, None, T::one(), &mut self.line_node.sink);
        self.line_node.sink.line_end();
        self.line_node.sink.polygon_end();
    }
}
