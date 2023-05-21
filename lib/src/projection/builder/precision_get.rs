use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleU;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionGet;
use crate::stream::Unconnected;

use super::Builder;

impl<PR, PCNC, PCNU, T> PrecisionGet
    for Builder<
        ClipAntimeridianU<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        PCNU,
        PR,
        Resample<PR, Unconnected, T>,
        T,
    >
where
    T: CoordFloat + Default + FloatConst,
{
    type T = T;

    #[inline]
    fn precision(&self) -> T {
        self.delta2.sqrt()
    }
}

impl<PR, PCNC, PCNU, T> PrecisionGet
    for Builder<
        ClipCircleU<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        PCNU,
        PR,
        Resample<PR, Unconnected, T>,
        T,
    >
where
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn precision(&self) -> T {
        self.delta2.sqrt()
    }
}
