use geo::CoordFloat;

use crate::projection::PrecisionGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> PrecisionGet
    for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    T: CoordFloat,
{
    type T = T;
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
