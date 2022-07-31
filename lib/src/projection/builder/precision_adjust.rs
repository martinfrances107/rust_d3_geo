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
        PCNU,
        PR,
        PV,
        Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>,
        Resample<PR, PCNC, Unconnected, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_set(mut self, delta: &T) -> Self {
        let delta2 = *delta * *delta;
        let resample = Resample::new(self.project_transform.clone(), delta2);
        self.delta2 = delta2;
        self.resample = resample;

        self
    }
}
