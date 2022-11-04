use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionAdjust;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNC, PCNU, PR, T> PrecisionAdjust
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU,
        PR,
        Resample<PR, ConnectedResample<PCNC, T>, T>,
        Resample<PR, Unconnected, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    PR: Clone + Transform<T = T>,
    PCNC: Clone,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_set(&mut self, delta: &T) -> &mut Self {
        self.base.precision_set(delta);
        self
    }
}
