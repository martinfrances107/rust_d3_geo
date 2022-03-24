use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use std::fmt::Debug;

// use crate::clip::Bufferable;
// use crate::clip::Interpolator;
// use crate::clip::LineConnected;
// use crate::clip::LineUnconnected;
// use crate::clip::PointVisible;

// use crate::clip::buffer::Buffer;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::Transform;
// use crate::projection::ProjectionRawBase;
use crate::projection::Translate;
// use crate::stream::Connectable;
use crate::stream::Connected;
// use crate::stream::Stream;
use crate::stream::Unconnected;

use super::Builder;
// use super::PostClipNode;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Translate
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        PCNC,
        PCNU,
        PR,
        PV,
        Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
        Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    DRAIN: Clone + Debug,
    I: Clone + Debug,
    PV: Clone + Debug,
    PCNC: Clone + Debug,
    PCNU: Clone + Debug,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    // I: Interpolator<
    //     EP = DRAIN,
    //     Stream = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
    //     T = T,
    // >,
    // DRAIN: Stream<EP = DRAIN, T = T> + Default,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>
    //     + Stream<EP = DRAIN, T = T>,
    // LU: LineUnconnected<SU = Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>>
    //     + Connectable<
    //         Output = LC,
    //         SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
    //     > + Bufferable<Output = LB, T = T>,
    // PCNC: PostClipNode + Stream<EP = DRAIN, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
    // PR: ProjectionRawBase<T>,
    PR: Transform<T = T>,
    // PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    fn translate(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Translate
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        PCNC,
        PCNU,
        PR,
        PV,
        ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
        ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    // I: Interpolator<
    //     EP = DRAIN,
    //     Stream = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
    //     T = T,
    // >,
    // DRAIN: Stream<EP = DRAIN, T = T> + Default,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>
    //     + Stream<EP = DRAIN, T = T>,
    // LU: LineUnconnected<SU = ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>>
    //     + Connectable<Output = LC, SC = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>
    //     + Bufferable<Output = LB, T = T>,
    // PCNC: PostClipNode + Stream<EP = DRAIN, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
    // PR: ProjectionRawBase<T>,
    // PV: PointVisible<T = T>,
    PR: Clone,
    LB: Clone,
    LU: Clone + Debug,
    PV: Clone + Debug,
    PCNU: Clone + Debug,
    DRAIN: Clone + Debug,
    LC: Clone + Debug,
    I: Clone,
    PR: Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    fn translate(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_no_resampling()
    }
}
