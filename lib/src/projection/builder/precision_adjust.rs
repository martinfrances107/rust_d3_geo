use geo::CoordFloat;

use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionAdjust;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, T> PrecisionAdjust
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, Resample<PR, Unconnected, T>, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat,
{
    type T = T;

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision_set(&mut self, delta: &T) -> &mut Self {
        let delta2 = *delta * *delta;
        let resample = Resample::new(self.project_transform.clone(), delta2);
        self.delta2 = delta2;
        self.resample = resample;
        self
    }
}
