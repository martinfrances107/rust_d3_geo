use geo::CoordFloat;
use geo::Coordinate;

use crate::projection::builder_mercator::Builder;
use crate::projection::CenterGet;

impl<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T> CenterGet
    for Builder<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn center(&self) -> Coordinate<T> {
        self.base.center()
    }
}
