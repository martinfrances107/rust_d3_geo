use core::fmt::Debug;
use core::marker::PhantomData;
use std::collections::VecDeque;

use geo::CoordFloat;
use geo::LineString;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::path::Result;
use crate::polygon_contains::polygon_contains;
use crate::stream::Connectable as StreamConnectable;
use crate::stream::Stream;
use crate::stream::Unconnected;

use super::compare_intersection::gen_compare;
use super::line_elem::LineElem;
use super::rejoin::rejoin;
use super::rejoin::CompareIntersectionsFn;
use super::Buffer;
use super::Bufferable;
use super::Clean;
use super::Interpolator;
use super::LineConnected;
use super::PointVisible;

/// Clip specific connections to a stream path.
pub trait Connectable {
    /// Represents to final connected state.
    type Output;
    /// The next stream node on the path.
    type SC;

    /// Connects to previous pathway stage.
    fn connect(&self, sink: Self::SC) -> Self::Output;
}

#[derive(Clone, Debug)]
enum PointFn {
    Default,
    Line,
    Ring,
}

#[derive(Clone, Debug)]
enum LineStartFn {
    Default,
    Ring,
}

#[derive(Clone, Debug)]
enum LineEndFn {
    Default,
    Ring,
}

/// Clip specific state of connection.
#[derive(Clone, Debug)]
pub struct Connected<LB, LC, T>
where
    T: CoordFloat,
{
    line_node: LC,
    polygon_started: bool,
    polygon: Vec<LineString<T>>,
    ring: LineString<T>,
    ring_sink: LB,
    segments: VecDeque<VecDeque<Vec<LineElem<T>>>>,
    point_fn: PointFn,
    line_start_fn: LineStartFn,
    line_end_fn: LineEndFn,
}

impl<I, LB, LC, LU, RC, T> Connectable for Clipper<I, LU, RC, Unconnected, T>
where
    I: Clone,
    LU: Clone
        + StreamConnectable<Output<RC> = LC>
        + Bufferable<LINE = LB, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type SC = RC;
    type Output = Clipper<I, LU, RC, Connected<LB, LC, T>, T>;

    fn connect(&self, sink: RC) -> Self::Output {
        let line_node = self.clip_line.clone().connect(sink);
        let ring_buffer = Buffer::<T>::default();
        let ring_sink = self.clip_line.clone().buffer(ring_buffer);
        let state = Connected {
            polygon_started: false,
            polygon: Vec::new(),
            ring_sink,
            ring: LineString(Vec::new()),
            segments: VecDeque::new(),
            line_node,
            point_fn: PointFn::Default,
            line_start_fn: LineStartFn::Default,
            line_end_fn: LineEndFn::Default,
        };

        Self::Output {
            p_rc: PhantomData::<RC>,
            state,
            clip_line: self.clip_line.clone(),
            interpolator: self.interpolator.clone(),
            start: self.start,
            compare: gen_compare(),
        }
    }
}

/// State associated with the clipping strategy.
///
/// Two distinct strategies [Antimeridian](crate::clip::antimeridian) and [Circle](crate::clip::circle).
pub struct Clipper<I, LU, RC, STATE, T>
where
    T: CoordFloat,
{
    state: STATE,
    /// The hidden linkage is in the implementation of Connectable
    /// Changing the RC results in a change of the Output type.
    p_rc: PhantomData<RC>,
    /// Needs to be public as `precision()` will copy these values.
    pub clip_line: LU,
    /// Antimerdian and Circle strategies have distinct interpolator functions.
    pub interpolator: I,
    /// First point checked in rejoin algorithm.
    pub start: Coord<T>,
    compare: CompareIntersectionsFn<T>,
}

impl<I, LU, RC, T> Clipper<I, LU, RC, Unconnected, T>
where
    T: 'static + CoordFloat + FloatConst,
{
    /// Takes a line and cuts into visible segments. Returns values used for polygon.
    pub fn new(interpolator: I, clip_line: LU, start: Coord<T>) -> Self {
        Self {
            p_rc: PhantomData::<RC>,
            state: Unconnected,
            clip_line,
            interpolator,
            start,
            compare: gen_compare(),
        }
    }
}

impl<I, LU, RC, STATE, T> Clone for Clipper<I, LU, RC, STATE, T>
where
    LU: Clone,
    I: Clone,
    STATE: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    /// Takes a line and cuts into visible segments. Returns values used for polygon.
    fn clone(&self) -> Self {
        Self {
            p_rc: PhantomData::<RC>,
            state: self.state.clone(),
            clip_line: self.clip_line.clone(),
            interpolator: self.interpolator.clone(),
            start: self.start,
            compare: gen_compare(),
        }
    }
}

// Had to manually implement because of compare is a closure.
impl<I, LU, RC, STATE, T> Debug for Clipper<I, LU, RC, STATE, T>
where
    STATE: Debug,
    LU: Debug,
    I: Debug,
    T: CoordFloat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rectangle")
            .field("state", &self.state)
            .field("clip_line", &self.clip_line)
            .field("interpolate", &self.interpolator)
            .field("start", &self.start)
            .finish_non_exhaustive()
    }
}

impl<EP, I, LB, LC, LU, RC, T> Clipper<I, LU, RC, Connected<LB, LC, T>, T>
where
    LB: LineConnected<SINK = Buffer<T>> + Clean + Stream<EP = Buffer<T>, T = T>,
    LC: LineConnected<SINK = RC> + PointVisible<T = T> + Stream<EP = EP, T = T>,
    LU: PointVisible<T = T>,
    RC: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    #[inline]
    fn line_end_default(&mut self) {
        self.state.point_fn = PointFn::Default;
        self.state.line_node.line_end();
    }

    #[inline]
    fn line_start_default(&mut self) {
        self.state.point_fn = PointFn::Line;
        self.state.line_node.line_start();
    }

    #[inline]
    fn point_default(&mut self, p: &Coord<T>, m: Option<u8>) {
        if self.clip_line.point_visible(p) {
            self.state.line_node.sink().point(p, m);
        }
    }

    #[inline]
    fn point_line(&mut self, p: &Coord<T>, m: Option<u8>) {
        self.state.line_node.point(p, m);
    }

    #[inline]
    fn point_ring(&mut self, p: &Coord<T>, m: Option<u8>) {
        self.state.ring.0.push(*p);
        self.state.ring_sink.point(p, m);
    }

    // rust_d3_geo_voronoi - profile_target flamegraph identifies this as a hot path!
    fn ring_end(&mut self) {
        let le = self.state.ring[0];
        // javascript version drops m here.
        self.point_ring(&le, None);
        self.state.ring_sink.line_end();

        let clean = self.state.ring_sink.clean();
        let ring_segments = self.state.ring_sink.sink();
        let n = ring_segments.lines.len();

        self.state.ring.0.pop();
        self.state.polygon.push(self.state.ring.clone());
        // in this javascript version this value is set to NULL
        // is my assumption that this is valid true?
        // self.ring = None;
        self.state.ring.0.clear();

        if n == 0 {
            return;
        }

        // No intersections.
        if clean & 1 != 0 {
            let segment = ring_segments.lines.pop_front().expect(
                "We have previously checked that the .len() is >0 ( n ) ",
            );
            let m = segment.len() - 1usize;
            if m > 0 {
                if !self.state.polygon_started {
                    self.state.line_node.sink().polygon_start();
                    self.state.polygon_started = true;
                }
                self.state.line_node.sink().line_start();
                for s in segment.iter().take(m) {
                    let point = s.p;
                    self.state.line_node.sink().point(&point, None);
                }
                self.state.line_node.sink().line_end();
            }
            return;
        }

        if n > 1 && (clean & 2 != 0) {
            ring_segments.rejoin();
        }

        ring_segments.lines.retain(|segment| segment.len() > 1usize);

        self.state.segments.push_back(ring_segments.result());
    }

    #[inline]
    fn ring_start(&mut self) {
        self.state.ring_sink.line_start();
        self.state.ring.0.clear();
    }
}

impl<EP, I, LB, LC, LU, RC, T> Stream
    for Clipper<I, LU, RC, Connected<LB, LC, T>, T>
where
    I: Interpolator<T = T>,
    LB: LineConnected<SINK = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: LineConnected<SINK = RC> + PointVisible<T = T> + Stream<EP = EP, T = T>,
    LU: PointVisible<T = T>,
    RC: Stream<EP = EP, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.line_node.sink().endpoint()
    }

    #[inline]
    fn line_end(&mut self) {
        match self.state.line_end_fn {
            LineEndFn::Default => {
                self.line_end_default();
            }
            LineEndFn::Ring => {
                self.ring_end();
            }
        }
    }

    #[inline]
    fn line_start(&mut self) {
        match self.state.line_start_fn {
            LineStartFn::Default => {
                self.line_start_default();
            }
            LineStartFn::Ring => {
                self.ring_start();
            }
        }
    }

    #[inline]
    fn point(&mut self, p: &Coord<T>, m: Option<u8>) {
        match self.state.point_fn {
            PointFn::Default => {
                self.point_default(p, m);
            }
            PointFn::Line => {
                self.point_line(p, m);
            }
            PointFn::Ring => {
                self.point_ring(p, m);
            }
        }
    }

    fn polygon_end(&mut self) {
        self.state.point_fn = PointFn::Default;
        self.state.line_start_fn = LineStartFn::Default;
        self.state.line_end_fn = LineEndFn::Default;
        // As tested with a single polygon...
        // When a polygon is completely visible segments is empty.
        // When a polygon is completely hidden segments is empty.
        // Only when the polygon is being clipped is segments populated.
        let mut segments: VecDeque<VecDeque<Vec<LineElem<T>>>> =
            VecDeque::default();
        core::mem::swap(&mut segments, &mut self.state.segments);
        let segments_inner: Vec<Vec<LineElem<T>>> =
            segments.into_iter().flatten().collect();
        let start_inside = polygon_contains(&self.state.polygon, &self.start);

        if !segments_inner.is_empty() {
            if !self.state.polygon_started {
                self.state.line_node.sink().polygon_start();
                self.state.polygon_started = true;
            }
            rejoin(
                &segments_inner,
                &self.compare,
                start_inside,
                &self.interpolator,
                self.state.line_node.sink(),
            );
        } else if start_inside {
            if !self.state.polygon_started {
                self.state.line_node.sink().polygon_start();
                self.state.polygon_started = true;
            }
            // Produce ring around the clipping circle!
            self.state.line_node.sink().line_start();
            self.interpolator.interpolate(
                None,
                None,
                T::one(),
                self.state.line_node.sink(),
            );
            self.state.ring_sink.sink().line_end();
        };
        if self.state.polygon_started {
            self.state.line_node.sink().polygon_end();
            self.state.polygon_started = false;
        }
        self.state.polygon.clear();
    }

    fn polygon_start(&mut self) {
        self.state.point_fn = PointFn::Ring;
        self.state.line_start_fn = LineStartFn::Ring;
        self.state.line_end_fn = LineEndFn::Ring;
        self.state.segments = VecDeque::new();
        self.state.polygon = Vec::new();
    }

    fn sphere(&mut self) {
        self.state.line_node.sink().polygon_start();
        self.state.line_node.sink().line_start();
        self.interpolator.interpolate(
            None,
            None,
            T::one(),
            self.state.line_node.sink(),
        );
        self.state.line_node.sink().line_end();
        self.state.line_node.sink().polygon_end();
    }
}
