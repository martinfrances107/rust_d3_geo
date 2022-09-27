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
        Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>,
        Resample<PR, PCNC, Unconnected, T>,
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
    fn precision_set(self, delta: &T) -> Self {
        let mut out = self;
        out.base = out.base.precision_set(delta);
        out
    }
}
