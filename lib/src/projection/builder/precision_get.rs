use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::Interpolator;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionGet;

use crate::stream::Unconnected;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> PrecisionGet
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
    I: Interpolator<
        EP = DRAIN,
        Stream = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
        T = T,
    >,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type Output = Builder<
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
    >;
    /// /**
    ///  * Returns the projection’s current resampling precision which defaults to square root of 0.5.
    ///  * This value corresponds to the Douglas–Peucker distance.
    ///  */
    /// /**
    ///  * Sets the threshold for the projection’s adaptive resampling to the specified value in PIxels and returns the projection.
    ///  * This value corresponds to the Douglas–Peucker distance.
    ///  *
    ///  * @param precision A numeric value in PIxels to use as the threshold for the projection’s adaptive resampling.
    ///  */
    #[inline]
    fn get_precision(&self) -> T {
        self.delta2.sqrt()
    }
}
