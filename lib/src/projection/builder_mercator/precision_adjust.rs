use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionAdjust;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, PCNU, PR, T> PrecisionAdjust
    for Builder<CLIPC, CLIPU, PCNU, PR, Resample<PR, Unconnected, T>, T>
where
    PR: Clone + Transform<T = T>,
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
