use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::PrecisionGet;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionGet
    for Builder<
        ClipAntimeridianC<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        ClipAntimeridianU<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        Resample<PR, ConnectedResample<PCNC, T>, T>,
        Resample<PR, Unconnected, T>,
        T,
    >
where
    PCNC: Clone,
    PCNU: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type T = T;

    #[inline]
    fn precision(&self) -> T {
        self.delta2.sqrt()
    }
}

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionGet
    for Builder<
        ClipCircleC<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        ClipCircleU<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        Resample<PR, ConnectedResample<PCNC, T>, T>,
        Resample<PR, Unconnected, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    PCNC: Clone,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn precision(&self) -> T {
        self.delta2.sqrt()
    }
}
