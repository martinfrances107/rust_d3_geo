use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use super::Builder;
use crate::projection::CenterGet;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> CenterGet
    for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_center(&self) -> Coordinate<T> {
        Coordinate {
            x: self.lambda.to_degrees(),
            y: self.phi.to_degrees(),
        }
    }
}
