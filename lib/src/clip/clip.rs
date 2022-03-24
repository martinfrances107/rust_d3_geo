use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use derivative::*;
use geo::CoordFloat;
use geo::Coordinate;
use geo::LineString;
use num_traits::FloatConst;

use crate::clip::Buffer;
use crate::clip::Clean;
use crate::clip::LineConnected;
use crate::path::Result;
use crate::polygon_contains::polygon_contains;
use crate::stream::Stream;
use crate::Transform;
use crate::stream::Connectable;
use crate::stream::Unconnected;

use super::compare_intersection::gen_compare_intersection;
use super::line_elem::LineElem;
use super::rejoin::rejoin;
use super::Bufferable;
use super::Interpolator;
use super::PointVisible;

#[derive(Clone, Debug)]
enum PointFn{
    Default,
    Line,
    Ring,
}

#[derive(Clone, Debug)]
enum LineStartFn{
    Default,
    Ring,
}

#[derive(Clone, Debug)]
enum LineEndFn{
    Default,
    Ring
}

/// Clip specific state of connection.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Connected<EP, LB, LC, LU, SC, SU, T>
where
    LB: Clone,
    T: CoordFloat,
{
    p_ep: PhantomData<EP>,
    p_sc: PhantomData<SC>,
    p_su: PhantomData<SU>,
    line_node: LC,
    p_lu: PhantomData<LU>,
    polygon_started: bool,
    polygon: Vec<LineString<T>>,
    ring: LineString<T>,
    ring_sink: LB,
    segments: VecDeque<VecDeque<Vec<LineElem<T>>>>,
    point_fn: PointFn,
    line_start_fn: LineStartFn,
    line_end_fn: LineEndFn,
}

impl<EP, LB, LC, LU, SC, SU, T> Connected<EP, LB, LC, LU, SC, SU, T>
where
    LB: Clone,
    LU: Clone + Connectable<Output = LC, SC = SC> + Bufferable<Output = LB, T = T>,
    T: CoordFloat,
{
    fn new(sink: SC, clip_line: LU) -> Self {
        let line_node = clip_line.clone().connect(sink);
        let ring_buffer = Buffer::<T>::default();
        let ring_sink = clip_line.buffer(ring_buffer);
        Self {
            p_lu: PhantomData::<LU>,
            p_su: PhantomData::<SU>,
            p_ep: PhantomData::<EP>,
            p_sc: PhantomData::<SC>,
            polygon_started: false,
            polygon: Vec::new(),
            // ring_buffer,
            ring_sink,
            ring: LineString(Vec::new()),
            segments: VecDeque::new(),
            // sink,
            line_node,
            point_fn: PointFn::Default,
            line_start_fn: LineStartFn::Default,
            line_end_fn: LineEndFn::Default,
            // use_point_default: true,
            // use_line_start_default: true,
            // use_line_end_default: true,
        }
    }
}

// pub struct Clip<EP, INTERPOLTATE, LINE, PV, SINK, STATE, T>
/// Takes the unconnected line temple stored in clip_line
/// and then modifies the ClipState to one than reflects
/// the connected sink.
impl<EP, I, LB, LC, LU, PR, PV, RC, RU, T> Connectable
    for Clip<EP, I, LB, LC, LU, PR, PV, RC, RU, Unconnected, T>
where
    LB: Clone,
    LU: Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T>,
    PR: Clone,
    LU: Clone,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type SC = RC;
    type Output = Clip<EP, I, LB, LC, LU, PR, PV, RC, RU, Connected<EP, LB, LC, LU, RC, RU, T>, T>;
    fn connect(self, sink: RC) -> Self::Output {
        Self::Output {
            p_ep: PhantomData::<EP>,
            p_lb: PhantomData::<LB>,
            p_lc: PhantomData::<LC>,
            p_pr: PhantomData::<PR>,
            p_rc: PhantomData::<RC>,
            p_ru: PhantomData::<RU>,
            state: Connected::new(sink, self.clip_line.clone()),
            clip_line: self.clip_line,
            interpolator: self.interpolator,
            pv: self.pv,
            start: self.start,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Clip<EP, I, LB, LC, LU, PR, PV, RC, RU, STATE, T>
where
    T: CoordFloat,
    LB: Clone,
{
    state: STATE,
    p_ep: PhantomData<EP>,
    p_lb: PhantomData<LB>,
    p_lc: PhantomData<LC>,
    p_pr: PhantomData<PR>,
    p_rc: PhantomData<RC>,
    p_ru: PhantomData<RU>,
    /// Needs to be public as precision() will copy these values.
    pub clip_line: LU,
    pub interpolator: I,
    pub pv: PV,
    pub start: Coordinate<T>,
}

impl<EP, I, LB, LC, LU, PR, PV, RC, RU, T>
    Clip<
        EP,
        I,
        LB,
        LC,
        LU,
        PR,
        PV,
        RC,
        RU,
        Unconnected,
        T,
    >
where
    LB: Clone,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// Takes a line and cuts into visible segments. Return values used for polygon
    ///
    ///
    /// Line<DRAIN, ResampleNoneNoClipU<DRAIN, PR, T>, Unconnected, T>
    pub fn new(interpolator: I, clip_line: LU, pv: PV, start: Coordinate<T>) -> Self {
        Clip {
            p_ep: PhantomData::<EP>,
            p_lb: PhantomData::<LB>,
            p_lc: PhantomData::<LC>,
            p_pr: PhantomData::<PR>,
            p_rc: PhantomData::<RC>,
            p_ru: PhantomData::<RU>,
            state: Unconnected,
            clip_line,
            interpolator,
            pv,
            start,
        }
    }
}

impl<EP, I, LB, LC, LU, PR, PV, RC, RU, T>
    Clip<EP, I, LB, LC, LU, PR, PV, RC, RU, Connected<EP, LB, LC, LU, RC, RU, T>, T>
where
    PV: PointVisible<T = T>,
    EP: Clone + Debug,
    I: Interpolator<EP = EP, Stream = RC, T = T>,
    LB: LineConnected<SC = Buffer<T>> + Clean + Stream<EP = Buffer<T>, T = T>,
    LC: LineConnected<SC = RC> + Stream<EP = EP, T = T>,
    LU: Clone + Debug + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T>,
    LC: LineConnected<SC = RC>,

    RC: Clone + Debug + Stream<EP = EP, T = T>,
    EP: Clone + Debug,
    I: Interpolator<EP = EP, Stream = RC, T = T>,
    LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: LineConnected<SC = RC> + Stream<EP = EP, T = T>,
    LU: Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T>,
    I: Interpolator<EP = EP, Stream = RC, T = T>,
    LC: Clone + Debug,
    LB: Clone + Debug + Stream<EP = Buffer<T>, T = T>,
    PR: Clone + Debug,
    PV: Clone + Debug,
    RC: Clone + Debug + Stream<EP = EP, T = T>,
    RU: Clone + Debug,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    #[inline]
    fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        if self.pv.point_visible(p) {
            self.state.line_node.point(p, m);
        }
    }

    #[inline]
    fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.state.line_node.point(p, m);
    }

    #[inline]
    fn line_start_default(&mut self) {
        self.state.point_fn = PointFn::Line;
        self.state.line_node.line_start();
    }

    #[inline]
    fn line_end_default(&mut self) {
        self.state.point_fn = PointFn::Default;
        self.state.line_node.line_end();
    }

    #[inline]
    fn point_ring(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        self.state.ring.0.push(*p);
        self.state.ring_sink.point(p, _m);
    }

    #[inline]
    fn ring_start(&mut self) {
        self.state.ring_sink.line_start();
        self.state.ring.0.clear();
    }

    fn ring_end(&mut self) {
        let le = self.state.ring[0];
        // javascript version drops m here.
        self.point_ring(&le, None);
        self.state.ring_sink.line_end();

        let clean = self.state.ring_sink.clean();
        let mut ring_segments = self.state.ring_sink.get_sink().result();
        let n = ring_segments.len();
        let m;

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
            let segment = ring_segments
                .pop_front()
                .expect("We have previously checked that the .len() is >0 ( n ) ");
            m = segment.len() - 1usize;
            if m > 0 {
                if !self.state.polygon_started {
                    self.state.line_node.get_sink().polygon_start();
                    self.state.polygon_started = true;
                }
                self.state.line_node.get_sink().line_start();
                for s in segment.iter().take(m) {
                    let point = s.p;
                    self.state.line_node.get_sink().point(&point, None);
                }
                self.state.line_node.get_sink().line_end();
            }
            return;
        }

        // Rejoin connected segments.
        // TODO reuse ringBuffer.rejoin()?
        if n > 1 {
            let pb = [
                ring_segments
                    .pop_back()
                    .unwrap_or_else(|| Vec::with_capacity(0)),
                ring_segments
                    .pop_front()
                    .unwrap_or_else(|| Vec::with_capacity(0)),
            ]
            .concat();
            ring_segments.push_back(pb);
        }

        ring_segments.retain(|segment| segment.len() > 1usize);

        self.state.segments.push_back(ring_segments);
    }
}

impl<EP, I, LB, LC, LU, PR, PV, RC, RU, T> Stream
    for Clip<EP, I, LB, LC, LU, PR, PV, RC, RU, Connected<EP, LB, LC, LU, RC, RU, T>, T>
where
    // EP: Stream<EP = EP, T = T> + Default,
    EP: Clone + Debug,
    I: Interpolator<EP = EP, Stream = RC, T = T>,
    LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: LineConnected<SC = RC> + Stream<EP = EP, T = T>,
    LU: Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T>,
    // PR: ProjectionRawBase<T>,
    PV: PointVisible<T = T>,
    // RC: Resampler + Stream<EP = EP, T = T>,
    // RU: Resampler,
    I: Interpolator<EP = EP, Stream = RC, T = T>,
    LC: Clone + Debug,
    LB: Clone + Debug + Stream<EP = Buffer<T>, T = T>,
    LU: Clone + Debug,
    PR: Clone + Debug,
    PV: Clone + Debug,
    RC: Clone + Debug + Stream<EP = EP, T = T>,
    RU: Clone + Debug,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type EP = EP;

    #[inline]
    fn get_endpoint(&mut self) -> &mut Self::EP {
        self.state.line_node.get_sink().get_endpoint()
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self.state.point_fn{
            PointFn::Default => {
                self.point_default(p, m);
            }
            PointFn::Line =>{
                self.point_line(p,m);
            }
            PointFn::Ring => {
                self.point_ring(p,m);
            }
        }

    }

    #[inline]
    fn line_start(&mut self) {
        match self.state.line_start_fn{
            LineStartFn::Default => {
                self.line_start_default();
            },
            LineStartFn::Ring => {
                self.ring_start();
            }
        }
    }

    #[inline]
    fn line_end(&mut self) {
        match self.state.line_end_fn {
            LineEndFn::Default => {
                self.line_end_default();
            },
            LineEndFn::Ring => {
                self.ring_end();
            },
        }
    }

    fn polygon_start(&mut self) {
        self.state.point_fn = PointFn::Ring;
        self.state.line_start_fn = LineStartFn::Ring;
        self.state.line_end_fn = LineEndFn::Ring;
        self.state.segments = VecDeque::new();
        self.state.polygon = Vec::new();
    }

    fn polygon_end(&mut self) {
        self.state.point_fn = PointFn::Default;
        self.state.line_start_fn = LineStartFn::Default;
        self.state.line_end_fn = LineEndFn::Default;
        let segments_inner: Vec<Vec<LineElem<T>>> =
            self.state.segments.clone().into_iter().flatten().collect();

        let start_inside = polygon_contains(&self.state.polygon, &self.start);

        if !segments_inner.is_empty() {
            self.state.line_node.get_sink().polygon_start();
            if !self.state.polygon_started {
                self.state.polygon_started = true;
            }
            rejoin(
                &segments_inner,
                gen_compare_intersection(),
                start_inside,
                &mut self.interpolator,
                self.state.line_node.get_sink(),
            );
        } else if start_inside {
            if !self.state.polygon_started {
                self.state.line_node.get_sink().polygon_start();
                self.state.polygon_started = true;
            }
            self.state.line_node.get_sink().line_start();
            self.interpolator
                .interpolate(None, None, T::one(), self.state.line_node.get_sink());
            self.state.ring_sink.get_sink().line_end();
        };
        if self.state.polygon_started {
            self.state.line_node.get_sink().polygon_end();
            self.state.polygon_started = false;
        }
        self.state.segments.clear();
        self.state.polygon.clear();
    }

    fn sphere(&mut self) {
        self.state.line_node.get_sink().polygon_start();
        self.state.line_node.get_sink().line_start();
        self.interpolator
            .interpolate(None, None, T::one(), self.state.line_node.get_sink());
        self.state.line_node.get_sink().line_end();
        self.state.line_node.get_sink().polygon_end();
    }
}
