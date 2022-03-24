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
// use crate::projection::ProjectionRawBase;
use crate::projection::Rotate;
// use crate::stream::Connectable;
use crate::stream::Connected;
// use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;
// use super::PostClipNode;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Rotate
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
    // PCNC: PostClipNode + Stream<EP = DRAIN, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
    // PR: ProjectionRawBase<T>,
    // PV: PointVisible<T = T>,
    DRAIN: Clone + Debug,
    I: Clone + Debug,
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

    #[inline]
    fn get_rotate(&self) -> [T; 3] {
        [
            self.delta_lambda.to_degrees(),
            self.delta_phi.to_degrees(),
            self.delta_lambda.to_degrees(),
        ]
    }

    /// Sets the rotation angles as measured in degrees.
    fn rotate(mut self, angles: &[T; 3]) -> Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Rotate
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
    DRAIN: Clone + Debug,
    I: Clone + Debug,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PCNU: Clone + Debug,
    PR: Transform<T = T>,
    PV: Clone + Debug,

    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_rotate(&self) -> [T; 3] {
        [
            self.delta_lambda.to_degrees(),
            self.delta_phi.to_degrees(),
            self.delta_lambda.to_degrees(),
        ]
    }

    /// Sets the rotation angles as measured in degrees.
    fn rotate(mut self, angles: &[T; 3]) -> Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter_no_resampling()
    }
}
