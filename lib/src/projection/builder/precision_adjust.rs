use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionAdjust;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

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
    DRAIN: Debug,
    LB: Debug,
    PCNC: Debug,
    PCNU: Debug,
    PR: Clone + Debug + Transform<T = T>,
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
