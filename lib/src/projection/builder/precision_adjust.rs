use geo::CoordFloat;

use super::Builder;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionAdjust;
use crate::stream::Unconnected;
use crate::Transform;

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
    PCNC: Clone,
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
