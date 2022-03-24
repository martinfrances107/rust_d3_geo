use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

// use crate::clip::buffer::Buffer;
// use crate::clip::Bufferable;
// use crate::clip::Interpolator;
// use crate::clip::LineConnected;
// use crate::clip::LineUnconnected;
// use crate::clip::PointVisible;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
// use crate::projection::ProjectionRawCommon;
// use crate::stream::Connectable;
use crate::stream::Connected;
// use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Angle;
use super::Builder;
// use super::PostClipNode;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Angle
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
    //     + Bufferable<Output = LB, T = T>
    //     + Connectable<
    //         Output = LC,
    //         SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
    //     >,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
    // PCNC: PostClipNode + Stream<EP = DRAIN, T = T>,
    // PR: ProjectionRawCommon<T>,
    // PV: PointVisible<T = T>,
    DRAIN: Clone + Debug,
    I: Clone,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PV: Clone + Debug,
    PCNU: Clone + Debug,
    PCNC: Clone + Debug,
    PR: Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    /// Returns the projection’s post-projection planar rotation angle.
    /// defaults to 0°.
    #[inline]
    fn get_angle(&self) -> Self::T {
        self.alpha.to_degrees()
    }

    /// Sets the projection’s post-projection planar rotation angle to the
    /// specified angle in degrees and returns the projection.
    ///
    fn angle(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Angle
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
    //     + Bufferable<Output = LB, T = T>
    //     + Connectable<Output = LC, SC = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
    // PCNC: PostClipNode + Stream<EP = DRAIN, T = T>,
    // PR: ProjectionRawCommon<T>,
    // PV: PointVisible<T = T>,
    // PR: ProjectionRawCommon<T>,
    // PV: PointVisible<T = T>,
    DRAIN: Clone + Debug,
    I: Clone,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PCNU: Clone + Debug,
    PV: Clone + Debug,
    PR: Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    /// Returns the projection’s post-projection planar rotation angle.
    /// defaults to 0°.
    #[inline]
    fn get_angle(&self) -> Self::T {
        self.alpha.to_degrees()
    }

    /// Sets the projection’s post-projection planar rotation angle to the
    /// specified angle in degrees and returns the projection.
    ///
    fn angle(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_no_resampling()
    }
}
