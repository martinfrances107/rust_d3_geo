use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

// use crate::clip::buffer::Buffer;
// use crate::clip::Bufferable;
// use crate::clip::Interpolator;
// use crate::clip::LineConnected;
use crate::Transform;
// use crate::clip::LineUnconnected;
// use crate::clip::PointVisible;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::ClipAngleGet;
// use crate::stream::Connectable;

// use crate::stream::Stream;
use crate::stream::Unconnected;

use super::Builder;
// use super::PostClipNode;
// use super::ProjectionRawBase;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> ClipAngleGet
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
    // DRAIN: Stream<EP = DRAIN, T = T> + Default,

    // I: Interpolator<
    //     EP = DRAIN,
    //     Stream = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
    //     T = T,
    // >,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>
    //     + Stream<EP = DRAIN, T = T>,
    // LU: LineUnconnected<SU = Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>>
    //     + Connectable<
    //         Output = LC,
    //         SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
    //     > + Bufferable<Output = LB, T = T>,
    // PR: ProjectionRawBase<T>,
    // PV: PointVisible<T = T>,
    // PCNC: PostClipNode + Stream<EP = DRAIN, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
    DRAIN: Clone + Debug,
    I: Clone,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PCNC: Clone + Debug,
    PCNU: Clone + Debug,
    PR: Transform<T = T>,
    PV: Clone + Debug,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    fn clip_angle_get(&self) -> T {
        self.theta.unwrap()
    }
}
