use crate::Transform;
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
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionAdjust;
// use crate::projection::ProjectionRawBase;
// use crate::stream::Connectable;
// use crate::stream::Stream;
use crate::stream::Unconnected;

use super::Builder;
// use super::PostClipNode;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> PrecisionAdjust
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
    DRAIN: Clone + Debug,
    I: Clone + Debug,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
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
    // PR: ProjectionRawBase<T>,
    PCNC: Clone + Debug,
    PCNU: Clone + Debug,
    PR: Clone + Debug + Transform<T = T>,
    PV: Clone + Debug,
    // PV: PointVisible<T = T>,
    // PCNC: PostClipNode,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_adjust(self, delta: &T) -> Self {
        let delta2 = *delta * *delta;
        let resample = Resample::new(self.project_transform.clone(), delta2);
        let out = Self {
            // Mutate section.
            delta2,
            resample,
            ..self
        };

        out.reset()
    }
}
